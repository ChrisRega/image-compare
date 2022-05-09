use image_compare::ssim_simple;
const RUN_COUNT: usize = 2000;
fn main() {
    let image_one = image::open("tests/data/pad_gaprao.png")
        .expect("Could not find test-image")
        .into_luma8();
    (0..RUN_COUNT).for_each(|_| {
        ssim_simple(&image_one, &image_one).unwrap();
    });
}
