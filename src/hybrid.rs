use crate::prelude::*;
use crate::utils::Window;
use crate::{gray_similarity_structure, Decompose};

fn merge_similarity_channels_yuv(input: &[GraySimilarityImage; 3]) -> RGBSimilarity {
    let mut image = RGBSimilarityImage::new(input[0].width(), input[0].height());
    let mut deviation = 0.0;
    Window::new((0, 0), (image.width() - 1, image.height() - 1))
        .iter_pixels()
        .for_each(|p| {
            let y = input[0].get_pixel(p.0, p.1)[0].clamp(0.0, 1.0);
            let u = input[1].get_pixel(p.0, p.1)[0].clamp(0.0, 1.0);
            let v = input[2].get_pixel(p.0, p.1)[0].clamp(0.0, 1.0);
            let color_diff = 1. - (1. - u.powi(2) + 1. - v.powi(2)).sqrt().clamp(0.0, 1.0);
            deviation += y.min(color_diff);

            image.put_pixel(p.0, p.1, Rgb([1. - y, 1. - u, 1. - v]))
        });
    let score = deviation as f64 / (image.width() as f64 * image.height() as f64);
    RGBSimilarity { image, score }
}

/// Comparing structure via MSSIM on Y channel, comparing color-diff-vectors on U and V summing the squares
/// Please mind that the RGBSimilarity-Image does _not_ contain plain RGB here
/// - The red channel contains 1. - similarity(ssim, y)
/// - The green channel contains 1. -  similarity(rms, u)
/// - The blue channel contains 1. -  similarity(rms, v)
/// This leads to a nice visualization of color and structure differences - with structural differences (meaning gray mssim diffs) leading to red rectangles
/// and and the u and v color diffs leading to color-deviations in green, blue and cyan
/// All-black meaning no differences
pub fn rgb_hybrid_compare(
    first: &RgbImage,
    second: &RgbImage,
) -> Result<RGBSimilarity, CompareError> {
    if first.dimensions() != second.dimensions() {
        return Err(CompareError::DimensionsDiffer);
    }

    let first_channels = first.split_to_yuv();
    let second_channels = second.split_to_yuv();
    let mssim_result = gray_similarity_structure(
        &Algorithm::MSSIMSimple,
        &first_channels[0],
        &second_channels[0],
    )?;
    let u_result = gray_similarity_structure(
        &Algorithm::RootMeanSquared,
        &first_channels[1],
        &second_channels[1],
    )?;
    let v_result = gray_similarity_structure(
        &Algorithm::RootMeanSquared,
        &first_channels[2],
        &second_channels[2],
    )?;

    let results = [mssim_result.image, u_result.image, v_result.image];

    Ok(merge_similarity_channels_yuv(&results))
}
