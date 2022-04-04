use crate::prelude::*;
use crate::utils;
use utils::Window;

pub fn root_mean_squared_error_simple(
    first: &GrayImage,
    second: &GrayImage,
) -> Result<Similarity, CompareError> {
    if first.dimensions() != second.dimensions() {
        return Err(CompareError::DimensionsDiffer);
    }
    let dimension = first.dimensions();
    let mut image = SimilarityImage::new(dimension.0, dimension.1);
    Window::new((0, 0), (dimension.0 - 1, dimension.1 - 1))
        .iter_pixels()
        .for_each(|pixel| {
            let diff = first.get_pixel(pixel.0, pixel.1)[0] as i32
                - second.get_pixel(pixel.0, pixel.1)[0] as i32;
            let normalized = diff as f32 / u8::MAX as f32;
            let squared = 1. - normalized.powi(2);

            image.put_pixel(pixel.0, pixel.1, Luma([squared]))
        });

    let score: f64 = 1.
        - (image.pixels().map(|p| 1. - p[0] as f64).sum::<f64>() / (image.pixels().len() as f64))
            .sqrt();
    Ok(Similarity { image, score })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rms_simple_10() {
        let mut first = GrayImage::new(1, 1);
        let mut second = GrayImage::new(1, 1);

        first.fill(0);
        second.fill(10);
        let comparison =
            root_mean_squared_error_simple(&first, &second).expect("Do not expect error here");
        assert_eq!(
            comparison.image.get_pixel(0, 0)[0],
            1. - (100. / (255.0f32.powi(2)))
        );
    }

    #[test]
    fn rms_simple_identity() {
        let mut first = GrayImage::new(1, 1);
        let mut second = GrayImage::new(1, 1);

        first.fill(0);
        second.fill(0);
        let comparison =
            root_mean_squared_error_simple(&first, &second).expect("Do not expect error here");
        assert_eq!(comparison.image.get_pixel(0, 0)[0], 1.);
        assert_eq!(comparison.score, 1.);
    }

    #[test]
    fn rms_simple_max() {
        let mut first = GrayImage::new(1, 1);
        let mut second = GrayImage::new(1, 1);

        first.fill(0);
        second.fill(255);
        let comparison =
            root_mean_squared_error_simple(&first, &second).expect("Do not expect error here");
        assert_eq!(comparison.image.get_pixel(0, 0)[0], 0.);
        assert_eq!(comparison.score, 0.);
    }

    #[test]
    fn rms_simple() {
        let width = 3;
        let height = 2;
        let mut first = GrayImage::new(width, height);
        let mut second = GrayImage::new(width, height);

        first.fill(0);
        second.fill(0);
        second.put_pixel(1, 1, Luma([127]));

        let comparison =
            root_mean_squared_error_simple(&first, &second).expect("Do not expect error here");

        let result = 1. - ((127. / 255.0f64).powi(2) / (width * height) as f64).sqrt();
        assert!((comparison.score - result).abs() < 1e-5);
    }
}
