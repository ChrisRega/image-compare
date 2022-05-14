use image_compare::{ssim_cached, ssim_simple};
const RUN_COUNT: usize = 200;
fn main() {
    let image_one = image::open("IMG_2741.jpg")
        .expect("Could not find test-image")
        .into_luma8();
    let image_two = image_one.clone();

    (0..RUN_COUNT).for_each(|_| {
        ssim_simple(&image_one, &image_two).unwrap();
    });
}
