#![crate_name = "image_compare"]
//! # Comparing images
//! This crate allows to compare grayscale images
//! The easiest use is loading two images, converting them to grayscale and running a comparison:
//! ```rust, no_run
//! use image_compare::Algorithm;
//! let image_one = image::open(first_image).expect("Could not find test-image").into_luma8();
//! let image_two = image::open(second_image).expect("Could not find test-image").into_luma8();
//! let result = image_compare::gray_similarity(Algorithm::MSSIMSimple, &image_one, &image_two).expect("Images had different dimensions");
//! ```
//!
//! Check the [`Algorithm`] enum for implementation details
//!
#![warn(missing_docs)]
#![warn(unused_qualifications)]
#![deny(deprecated)]

mod squared_error;
mod ssim;
mod utils;

#[doc(hidden)]
pub mod prelude {
    pub use image::{GrayImage, ImageBuffer, Luma};
    use thiserror::Error;

    /// The enum for selecting a grayscale comparison implementation
    pub enum Algorithm {
        /// A simple RMSE implementation - will return: <img src="https://render.githubusercontent.com/render/math?math=1-\sqrt{\frac{(\sum_{x,y=0}^{x,y=w,h}\left(f(x,y)-g(x,y)\right)^2)}{w*h}}">
        RootMeanSquared,
        /// a simple MSSIM implemenation - will run SSIM (implemented as on wikipedia) over 8x8 px windows and average the results
        MSSIMSimple,
    }

    #[derive(Error, Debug)]
    /// The errors that can occur during comparison of the images
    pub enum CompareError {
        #[error("The dimensions of the input images are not identical")]
        DimensionsDiffer,
    }

    /// a single-channel f32 typed image containing a result-score for each pixel
    pub type SimilarityImage = ImageBuffer<Luma<f32>, Vec<f32>>;

    #[derive(Debug)]
    /// A struct containing the results of a grayscale comparison
    pub struct Similarity {
        /// Contains the resulting differences per pixel.
        /// The buffer will contain the resulting values of the respective algorithms:
        /// - RMS will be between 0. for all-white vs all-black and 1.0 for identical
        /// - SSIM usually is near 1. for similar, near 0. for different but can take on negative values for negative covariances
        pub image: SimilarityImage,
        /// the averaged resulting score
        pub score: f64,
    }
}
#[doc(inline)]
pub use prelude::Algorithm;
#[doc(inline)]
pub use prelude::CompareError;
#[doc(inline)]
pub use prelude::Similarity;
use prelude::*;

/// The current main function of the crate
///
/// # Arguments
///
/// * `algorithm` - The comparison algorithm to use
///
/// * `first` - The first of the images to compare
///
/// * `second` - The first of the images to compare
pub fn gray_similarity(
    algorithm: Algorithm,
    first: &GrayImage,
    second: &GrayImage,
) -> Result<Similarity, CompareError> {
    match algorithm {
        Algorithm::RootMeanSquared => squared_error::root_mean_squared_error_simple(first, second),
        Algorithm::MSSIMSimple => ssim::ssim_simple(first, second),
    }
}
