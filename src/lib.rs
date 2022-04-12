#![crate_name = "image_compare"]
//! # Comparing gray images using structure
//! This crate allows to compare grayscale images using either structure or histogramming methods.
//! The easiest use is loading two images, converting them to grayscale and running a comparison:
//! ```no_run
//! use image_compare::Algorithm;
//! let image_one = image::open("image1.png").expect("Could not find test-image").into_luma8();
//! let image_two = image::open("image2.png").expect("Could not find test-image").into_luma8();
//! let result = image_compare::gray_similarity_structure(Algorithm::MSSIMSimple, &image_one, &image_two).expect("Images had different dimensions");
//! ```
//! Check the [`Algorithm`] enum for implementation details
//!
//! # Comparing gray images using histogram
//!
//! Histogram comparisons are possible using the histogram comparison function
//! ```no_run
//! use image_compare::Metric;
//! let image_one = image::open("image1.png").expect("Could not find test-image").into_luma8();
//! let image_two = image::open("image2.png").expect("Could not find test-image").into_luma8();
//! let result = image_compare::gray_similarity_histogram(Metric::Hellinger, &image_one, &image_two).expect("Images had different dimensions");
//! ```
//! //! Check the [`Metric`] enum for implementation details
#![warn(missing_docs)]
#![warn(unused_qualifications)]
#![deny(deprecated)]

mod histogram;
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
        /// a simple MSSIM implementation - will run SSIM (implemented as on wikipedia: <img src="https://render.githubusercontent.com/render/math?math=\mathrm{SSIM}(x,y)={\frac {(2\mu _{x}\mu _{y}+c_{1})(2\sigma _{xy}+c_{2})}{(\mu _{x}^{2}+\mu _{y}^{2}+c_{1})(\sigma _{x}^{2}+\sigma _{y}^{2}+c_{2})}}"> ) over 8x8 px windows and average the results
        MSSIMSimple,
    }

    #[derive(Error, Debug)]
    /// The errors that can occur during comparison of the images
    pub enum CompareError {
        #[error("The dimensions of the input images are not identical")]
        DimensionsDiffer,
        #[error("Comparison calculation failed: {0}")]
        CalculationFailed(String),
    }

    /// a single-channel f32 typed image containing a result-score for each pixel
    pub type SimilarityImage = ImageBuffer<Luma<f32>, Vec<f32>>;

    #[derive(Debug)]
    /// A struct containing the results of a grayscale comparison
    pub struct Similarity {
        /// Contains the resulting differences per pixel if applicable
        /// The buffer will contain the resulting values of the respective algorithms:
        /// - RMS will be between 0. for all-white vs all-black and 1.0 for identical
        /// - SSIM usually is near 1. for similar, near 0. for different but can take on negative values for negative covariances
        pub image: SimilarityImage,
        /// the averaged resulting score
        pub score: f64,
    }

    pub trait ToGrayScale {
        /// Clamps each input pixel to (0., 1.) and multiplies by 255 before converting to u8.
        /// See tests/data/*_compare.png images for examples
        fn to_grayscale(&self) -> GrayImage;
    }

    impl ToGrayScale for SimilarityImage {
        fn to_grayscale(&self) -> GrayImage {
            let mut img_gray = GrayImage::new(self.width(), self.height());
            for row in 0..self.height() {
                for col in 0..self.width() {
                    let new_val = self.get_pixel(col, row)[0].clamp(0., 1.) * 255.;
                    img_gray.put_pixel(col, row, Luma([new_val as u8]));
                }
            }
            img_gray
        }
    }
}
#[doc(inline)]
pub use histogram::Metric;
#[doc(inline)]
pub use prelude::Algorithm;
#[doc(inline)]
pub use prelude::CompareError;
#[doc(inline)]
pub use prelude::Similarity;
#[doc(inline)]
pub use prelude::SimilarityImage;
pub use prelude::ToGrayScale;
use prelude::*;

/// Comparing gray images using structure.
///
/// # Arguments
///
/// * `algorithm` - The comparison algorithm to use
///
/// * `first` - The first of the images to compare
///
/// * `second` - The first of the images to compare
pub fn gray_similarity_structure(
    algorithm: Algorithm,
    first: &GrayImage,
    second: &GrayImage,
) -> Result<Similarity, CompareError> {
    if first.dimensions() != second.dimensions() {
        return Err(CompareError::DimensionsDiffer);
    }
    match algorithm {
        Algorithm::RootMeanSquared => squared_error::root_mean_squared_error_simple(first, second),
        Algorithm::MSSIMSimple => ssim::ssim_simple(first, second),
    }
}

/// Comparing gray images using histogram
/// # Arguments
///
/// * `metric` - The distance metric to use
///
/// * `first` - The first of the images to compare
///
/// * `second` - The first of the images to compare
pub fn gray_similarity_histogram(
    metric: Metric,
    first: &GrayImage,
    second: &GrayImage,
) -> Result<f64, CompareError> {
    if first.dimensions() != second.dimensions() {
        return Err(CompareError::DimensionsDiffer);
    }
    histogram::img_compare(first, second, metric)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dimensions_differ_test() {
        let first = GrayImage::new(1, 1);
        let second = GrayImage::new(2, 2);
        let result = gray_similarity_structure(Algorithm::RootMeanSquared, &first, &second);
        assert!(result.is_err());
    }
}
