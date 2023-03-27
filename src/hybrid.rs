use crate::colorization::{GraySimilarityImage, RGBASimilarityImage, RGBSimilarityImage};
use crate::prelude::*;
use crate::squared_error::root_mean_squared_error_simple;
use crate::ssim::ssim_simple;
use crate::utils::{blend_alpha, split_rgba_to_yuva};
use crate::Decompose;
use image::{Rgba, RgbaImage};
use itertools::izip;

fn merge_similarity_channels_yuva(
    input: &[GraySimilarityImage; 4],
    alpha: &GrayImage,
    alpha_second: &GrayImage,
) -> Similarity {
    const ALPHA_VIS_MIN: f32 = 0.1;

    let mut image = RGBASimilarityImage::new(input[0].width(), input[0].height());
    let mut deviation = Vec::new();
    deviation.resize((input[0].width() * input[0].height()) as usize, 0.0);
    izip!(
        image.pixels_mut(),
        input[0].pixels(),
        input[1].pixels(),
        input[2].pixels(),
        input[3].pixels(),
        alpha.pixels(),
        alpha_second.pixels(),
        deviation.iter_mut()
    )
    .for_each(
        |(rgba, y, u, v, a_d, alpha_source, alpha_source_second, deviation)| {
            let y = y[0].clamp(0.0, 1.0);
            let u = u[0].clamp(0.0, 1.0);
            let v = v[0].clamp(0.0, 1.0);
            let a_d = a_d[0].clamp(0.0, 1.0);
            let alpha_bar = (alpha_source[0] as f32 + alpha_source_second[0] as f32) / (2. * 255.);
            let alpha_bar = if alpha_bar.is_finite() {
                alpha_bar
            } else {
                1.0
            };

            let color_diff = ((u).powi(2) + (v).powi(2)).sqrt().clamp(0.0, 1.0);
            let min_sim = y.min(color_diff).min(a_d);
            //the lower the alpha the less differences are visible in color and structure (and alpha)

            let dev = if alpha_bar > 0. {
                (min_sim / alpha_bar).clamp(0., 1.)
            } else {
                1.0
            };
            let alpha_vis = (ALPHA_VIS_MIN + a_d * (1.0 - ALPHA_VIS_MIN)).clamp(0., 1.);

            *deviation = dev;
            *rgba = Rgba([1. - y, 1. - u, 1. - v, alpha_vis]);
        },
    );

    let score = deviation.iter().sum::<f32>() as f64 / deviation.len() as f64;
    Similarity {
        image: Some(image.into()),
        score,
    }
}

fn merge_similarity_channels_yuv(input: &[GraySimilarityImage; 3]) -> Similarity {
    let mut image = RGBSimilarityImage::new(input[0].width(), input[0].height());
    let mut deviation = Vec::new();
    deviation.resize((input[0].width() * input[0].height()) as usize, 0.0);
    izip!(
        image.pixels_mut(),
        input[0].pixels(),
        input[1].pixels(),
        input[2].pixels(),
        deviation.iter_mut()
    )
    .for_each(|(rgb, y, u, v, deviation)| {
        let y = y[0].clamp(0.0, 1.0);
        let u = u[0].clamp(0.0, 1.0);
        let v = v[0].clamp(0.0, 1.0);
        let color_diff = ((u).powi(2) + (v).powi(2)).sqrt().clamp(0.0, 1.0);
        //f32 for keeping numerical stability for hybrid compare in 0.2.-branch
        *deviation += y.min(color_diff);
        *rgb = Rgb([1. - y, 1. - u, 1. - v]);
    });

    let score = deviation.iter().sum::<f32>() as f64 / deviation.len() as f64;
    Similarity {
        image: Some(image.into()),
        score,
    }
}

/// Hybrid comparison for RGBA images.
/// Will do MSSIM on luma, then RMS on U and V and alpha channels.
/// The calculation of the score is then pixel-wise the minimum of each pixels similarity.
/// To account for perceived indifference in lower alpha regions, this down-weights the difference
/// linearly with mean alpha channel.
pub fn rgba_hybrid_compare(
    first: &RgbaImage,
    second: &RgbaImage,
) -> Result<Similarity, CompareError> {
    if first.dimensions() != second.dimensions() {
        return Err(CompareError::DimensionsDiffer);
    }

    let first = split_rgba_to_yuva(first);
    let second = split_rgba_to_yuva(second);

    let (_, mssim_result) = ssim_simple(&first[0], &second[0])?;
    let (_, u_result) = root_mean_squared_error_simple(&first[1], &second[1])?;
    let (_, v_result) = root_mean_squared_error_simple(&first[2], &second[2])?;

    let (_, alpha_result) = root_mean_squared_error_simple(&first[3], &second[3])?;

    let results = [mssim_result, u_result, v_result, alpha_result];

    Ok(merge_similarity_channels_yuva(
        &results, &first[3], &second[3],
    ))
}

/// This processes the RGBA images be pre-blending the colors with the desired background color.
/// It's faster then the full RGBA similarity and more intuitive.
pub fn rgba_blended_hybrid_compare(
    first: &RgbaImage,
    second: &RgbaImage,
    background: Rgb<u8>,
) -> Result<Similarity, CompareError> {
    let first = blend_alpha(first, background);
    let second = blend_alpha(second, background);
    rgb_hybrid_compare(&first, &second)
}

/// Comparing structure via MSSIM on Y channel, comparing color-diff-vectors on U and V summing the squares
/// Please mind that the RGBSimilarity-Image does _not_ contain plain RGB here
/// - The red channel contains 1. - similarity(ssim, y)
/// - The green channel contains 1. -  similarity(rms, u)
/// - The blue channel contains 1. -  similarity(rms, v)
/// This leads to a nice visualization of color and structure differences - with structural differences (meaning gray mssim diffs) leading to red rectangles
/// and and the u and v color diffs leading to color-deviations in green, blue and cyan
/// All-black meaning no differences
pub fn rgb_hybrid_compare(first: &RgbImage, second: &RgbImage) -> Result<Similarity, CompareError> {
    if first.dimensions() != second.dimensions() {
        return Err(CompareError::DimensionsDiffer);
    }

    let first_channels = first.split_to_yuv();
    let second_channels = second.split_to_yuv();
    let (_, mssim_result) = ssim_simple(&first_channels[0], &second_channels[0])?;
    let (_, u_result) = root_mean_squared_error_simple(&first_channels[1], &second_channels[1])?;
    let (_, v_result) = root_mean_squared_error_simple(&first_channels[2], &second_channels[2])?;

    let results = [mssim_result, u_result, v_result];

    Ok(merge_similarity_channels_yuv(&results))
}
