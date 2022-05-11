use image_compare::{ssim_cached, ssim_simple};
const RUN_COUNT: usize = 2000;
fn main() {
    let image_one = image::open("tests/data/pad_gaprao.png")
        .expect("Could not find test-image")
        .into_luma8();
    let image_two = image_one.clone();

    (0..RUN_COUNT).for_each(|_| {
        ssim_cached(&image_one, &image_two).unwrap();
    });
}
