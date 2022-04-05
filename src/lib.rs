mod squared_error;
mod ssim;
mod utils;

pub mod prelude {
    pub use image::{GrayImage, ImageBuffer, Luma};
    use thiserror::Error;

    pub enum Algorithm {
        RootMeanSquared,
        MSSIMSimple,
    }

    #[derive(Error, Debug)]
    pub enum CompareError {
        #[error("The dimensions of the input images are not identical")]
        DimensionsDiffer,
    }

    pub type SimilarityImage = ImageBuffer<Luma<f32>, Vec<f32>>;

    #[derive(Debug)]
    pub struct Similarity {
        pub image: SimilarityImage,
        pub score: f64,
    }
}
use prelude::*;

pub fn similarity(
    algorithm: Algorithm,
    first: &GrayImage,
    second: &GrayImage,
) -> Result<Similarity, CompareError> {
    match algorithm {
        Algorithm::RootMeanSquared => squared_error::root_mean_squared_error_simple(first, second),
        Algorithm::MSSIMSimple => ssim::ssim_simple(first, second),
    }
}
