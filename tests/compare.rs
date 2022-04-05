use std::convert::Infallible;

use async_trait::async_trait;
use cucumber::{given, then, when, World, WorldInit};
use image_compare::prelude::*;
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
        image_compare::similarity(
            Algorithm::RootMeanSquared,
            world.first.as_ref().unwrap(),
            world.second.as_ref().unwrap(),
        )
        .expect("Error comparing the two images!"),
    );
}

#[when(expr = "comparing the images using MSSIM")]
fn compare_mssim(world: &mut CompareWorld) {
    world.comparison_result = Some(
        image_compare::similarity(
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
    CompareWorld::run("tests/features/compare.feature").await;
}
