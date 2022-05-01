use crate::prelude::*;
use crate::utils::{draw_window_to_image, Window};
use rayon::prelude::*;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

const DEFAULT_WINDOW_SIZE: u32 = 8;
const K1: f64 = 0.01;
const K2: f64 = 0.03;
const L: u8 = u8::MAX;
const C1: f64 = (K1 * L as f64) * (K1 * L as f64);
const C2: f64 = (K2 * L as f64) * (K2 * L as f64);

pub fn ssim_simple(first: &GrayImage, second: &GrayImage) -> Result<GraySimilarity, CompareError> {
    let dimension = first.dimensions();
    let mut image = GraySimilarityImage::new(dimension.0, dimension.1);
    let window = Window::from_image(first);
    let windows = window.subdivide_by_offset(DEFAULT_WINDOW_SIZE);
    let results = windows
        .par_iter()
        .map(|w| (ssim_for_window(first, second, &w), w))
        .collect::<Vec<_>>();
    let score = results.iter().map(|r| r.0 * r.1.area() as f64).sum::<f64>()
        / results.iter().map(|r| r.1.area() as f64).sum::<f64>();

    results
        .iter()
        .for_each(|r| draw_window_to_image(r.1, &mut image, r.0 as f32));

    Ok(GraySimilarity { image, score })
}

fn ssim_for_window(first: &GrayImage, second: &GrayImage, window: &Window) -> f64 {
    let mean_x = mean_simd(first, window);
    let mean_y = mean_simd(second, window);
    let variance_x = covariance_simd(first, mean_x, first, mean_x, window);
    let variance_y = covariance_simd(second, mean_y, second, mean_y, window);
    let covariance = covariance_simd(first, mean_x, second, mean_y, window);
    let counter = (2. * mean_x * mean_y + C1) * (2. * covariance + C2);
    let denominator = (mean_x.powi(2) + mean_y.powi(2) + C1) * (variance_x + variance_y + C2);
    counter / denominator
}

fn covariance(
    image_x: &GrayImage,
    mean_x: f64,
    image_y: &GrayImage,
    mean_y: f64,
    window: &Window,
) -> f64 {
    window
        .iter_pixels()
        .map(|pixel| {
            let pixel_x: f64 = image_x.get_pixel(pixel.0, pixel.1)[0].into();
            let pixel_y: f64 = image_y.get_pixel(pixel.0, pixel.1)[0].into();

            (pixel_x - mean_x) * (pixel_y - mean_y)
        })
        .sum::<f64>()
}

fn covariance_simd(
    image_x: &GrayImage,
    mean_x: f64,
    image_y: &GrayImage,
    mean_y: f64,
    window: &Window,
) -> f64 {
    if window.width() != 8 {
        return covariance(image_x, mean_x, image_y, mean_y, window);
    }

    let mut sum: f64 = 0.0;
    for row in 0..window.height() {
        let mean_x_f32 = mean_x as f32;
        let mean_y_f32 = mean_y as f32;
        unsafe {
            let row = row + window.top_left.1;
            let row_floats_x = load_as_float(&image_x.get_pixel(window.top_left.0, row).0);
            let row_floats_mean_x = _mm256_set1_ps(mean_x_f32);
            let row_floats_y = load_as_float(&image_y.get_pixel(window.top_left.0, row).0);
            let row_floats_mean_y = _mm256_set1_ps(mean_y_f32);

            let diffs_x = _mm256_sub_ps(row_floats_x, row_floats_mean_x);
            let diffs_y = _mm256_sub_ps(row_floats_y, row_floats_mean_y);
            let cov = _mm256_mul_ps(diffs_x, diffs_y);
            let cov_sum = sum_reduce(cov);
            sum += cov_sum as f64;
        }
    }
    sum
}

fn mean(image: &GrayImage, window: &Window) -> f64 {
    let mut result = 0.0;
    let mut area: usize = 0;

    window.iter_pixels().for_each(|pixel| {
        if let Some(pixel) = image.get_pixel_checked(pixel.0, pixel.1) {
            result += pixel[0] as f64;
            area += 1;
        }
    });

    result / area as f64
}

#[inline(always)]
fn sum_reduce(rows: __m256) -> f32 {
    unsafe {
        let hi_quad = _mm256_extractf128_ps::<1>(rows);
        let lo_quad = _mm256_castps256_ps128(rows);
        let sum_quad = _mm_add_ps(lo_quad, hi_quad);
        let lo_dual = sum_quad;
        let hi_dual = _mm_movehl_ps(sum_quad, sum_quad);
        let sum_dual = _mm_add_ps(lo_dual, hi_dual);
        let lo = sum_dual;
        let hi = _mm_shuffle_ps::<0x1>(sum_dual, sum_dual);
        let sum = _mm_add_ss(lo, hi);
        _mm_cvtss_f32(sum)
    }
}

#[inline(always)]
fn load_as_float(px: &[u8]) -> __m256 {
    unsafe {
        let row = _mm_loadu_si128(px.as_ptr() as *const _);
        let row_wide = _mm256_cvtepu8_epi32(row);
        _mm256_cvtepi32_ps(row_wide)
    }
}

fn mean_simd(image: &GrayImage, window: &Window) -> f64 {
    if window.width() != 8 {
        return mean(image, window);
    }

    let mut sum = 0.0;
    for row in 0..window.height() {
        let row = row + window.top_left.1;
        let row_floats = load_as_float(&image.get_pixel(window.top_left.0, row).0);
        let result = sum_reduce(row_floats);
        sum += result;
    }

    sum as f64 / window.area() as f64
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn conv_avg_simple() {
        let mut img = GrayImage::new(1, 1);
        img.fill(100);
        let window = Window::new((0, 0), (0, 0));
        let conv_result = mean(&img, &window);
        assert_eq!(conv_result, 100.);
    }

    #[test]
    fn conv_avg_identity() {
        let window = Window::new((0, 0), (9, 9));
        let mut img = GrayImage::new(window.width(), window.height());
        img.fill(1);
        let conv_result = mean(&img, &window);
        assert!((conv_result - 1.).abs() < 1e-5);
    }

    #[test]
    fn test_variance() {
        let window = Window::new((0, 0), (2, 2));
        let mut img = GrayImage::new(window.width(), window.height());
        img.fill(0);
        img.put_pixel(1, 1, Luma([9]));
        let avg = mean(&img, &window);
        let var = covariance(&img, avg, &img, avg, &window);
        assert_eq!(avg, 1.0);
        //64. by the central piece being (9-1)^2, 8 in total by each element contributing (0-1)^2
        assert_eq!(var, 64. + 8.);
    }

    #[test]
    fn test_performance() {
        let image_one = image::open("tests/data/pad_gaprao.png")
            .expect("Could not find test-image")
            .into_luma8();
        const RUNCOUNT: usize = 10;
        (0..RUNCOUNT).for_each(|_| {
            ssim_simple(&image_one, &image_one).unwrap();
        });
    }

    #[test]
    fn test_ssim_identity() {
        let window = Window::new((0, 0), (DEFAULT_WINDOW_SIZE - 1, DEFAULT_WINDOW_SIZE - 1));
        let mut img = GrayImage::new(window.width(), window.height());
        img.fill(0);
        img.put_pixel(1, 1, Luma([9]));
        let ssim_value = ssim_for_window(&img, &img, &window);
        assert_eq!(ssim_value, 1.0);
    }

    #[test]
    fn test_ssim_stability() {
        let window = Window::new((0, 0), (2, 2));
        let mut img = GrayImage::new(window.width(), window.height());
        img.fill(0);
        let ssim_value = ssim_for_window(&img, &img, &window);
        assert_eq!(ssim_value, 1.0);
    }

    #[test]
    fn test_ssim_different_mean() {
        let window = Window::new((0, 0), (2, 2));
        let mut img = GrayImage::new(window.width(), window.height());
        img.fill(0);
        let mut img_second = GrayImage::new(window.width(), window.height());
        img_second.fill(5);

        let ssim_value = ssim_for_window(&img, &img_second, &window);
        assert_eq!(ssim_value, 0.20641218950876916);
    }
}
