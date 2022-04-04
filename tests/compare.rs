use std::convert::Infallible;

use async_trait::async_trait;
use cucumber::{given, World, WorldInit};
use image_compare::prelude::*;

// `World` is your shared, likely mutable state.
#[derive(Debug, WorldInit)]
pub struct ImageCompareWorldClass {
    first: Option<GrayImage>,
    second: Option<GrayImage>,
    comparison_result: Option<Similarity>,
}

// `World` needs to be implemented, so Cucumber knows how to construct it
// for each scenario.
#[async_trait(?Send)]
impl World for ImageCompareWorldClass {
    // We do require some error type.
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
    ImageCompareWorldClass::run("tests/features/compare").await;
}
