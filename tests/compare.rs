use std::convert::Infallible;

use async_trait::async_trait;
use cucumber::{given, then, when, World, WorldInit};
use image_compare::prelude::*;
use image_compare::Metric;
extern crate image;

// `World` is your shared, likely mutable state.
#[derive(Debug, WorldInit)]
pub struct CompareWorld {
    first: Option<GrayImage>,
    second: Option<GrayImage>,
    comparison_result: Option<Similarity>,
}

#[given(expr = "the images {string} and {string} are loaded")]
fn load_images(world: &mut CompareWorld, first_image: String, second_image: String) {
    let image_one = image::open(first_image).expect("Could not find test-image");
    let image_two = image::open(second_image).expect("Could not find test-image");
    let image_one_gray = image_one.into_luma8();
    let image_two_gray = image_two.into_luma8();
    world.first = Some(image_one_gray);
    world.second = Some(image_two_gray);
}

#[when(expr = "comparing the images using RMS")]
fn compare_rms(world: &mut CompareWorld) {
    world.comparison_result = Some(
        image_compare::gray_similarity_structure(
            Algorithm::RootMeanSquared,
            world.first.as_ref().unwrap(),
            world.second.as_ref().unwrap(),
        )
        .expect("Error comparing the two images!"),
    );
}

#[when(expr = "comparing the images using histogram {string}")]
fn compare_hist_corr(world: &mut CompareWorld, metric: String) {
    let metric = match metric.as_str() {
        "correlation" => Metric::Correlation,
        "chisquare" => Metric::ChiSquare,
        "intersection" => Metric::Intersection,
        "hellinger distance" => Metric::Hellinger,
        _ => panic!(),
    };
    world.comparison_result = Some(Similarity {
        score: image_compare::gray_similarity_histogram(
            metric,
            world.first.as_ref().unwrap(),
            world.second.as_ref().unwrap(),
        )
        .expect("Error comparing the two images!"),
        image: SimilarityImage::new(1, 1),
    });
}

#[when(expr = "comparing the images using MSSIM")]
fn compare_mssim(world: &mut CompareWorld) {
    world.comparison_result = Some(
        image_compare::gray_similarity_structure(
            Algorithm::MSSIMSimple,
            world.first.as_ref().unwrap(),
            world.second.as_ref().unwrap(),
        )
        .expect("Error comparing the two images!"),
    );
}

#[then(expr = "the similarity score is {float}")]
fn check_result_score(world: &mut CompareWorld, score: f64) {
    assert_eq!(world.comparison_result.as_ref().unwrap().score, score);
}

#[then(expr = "the similarity image matches {string}")]
fn check_result_image(world: &mut CompareWorld, reference: String) {
    let img = world
        .comparison_result
        .as_ref()
        .unwrap()
        .image
        .to_grayscale();
    let image_one = image::open(reference)
        .expect("Could not find reference-image")
        .into_luma8();
    assert_eq!(
        image_compare::gray_similarity_structure(Algorithm::RootMeanSquared, &img, &image_one)
            .expect("Could not compare")
            .score,
        1.0
    );
}

#[async_trait(?Send)]
impl World for CompareWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            first: None,
            second: None,
            comparison_result: None,
        })
    }
}

#[tokio::main]
async fn main() {
    CompareWorld::run("tests/features/structure_gray.feature").await;
    CompareWorld::run("tests/features/histogram_gray.feature").await;
}
