extern crate image;
extern crate rmr;

use image::{DynamicImage, GenericImage, ImageFormat};

use rmr::carve::Carver;

use std::fs::File;

macro_rules! test_carve {
    ( $target:expr, $dw:expr, $dh:expr ) => {
        let input = load(INPUT);

        let (width, height) = input.dimensions();
        let target_width = (width as isize + $dw) as usize;
        let target_height = (height as isize + $dh) as usize;

        let mut carver = Carver::new(&input);
        let output = carver.resize(target_width, target_height);

        let target = load($target);
        if let Err(msg) = compare_images(&target, &output) {
            let filename = format!("{}.png", stringify!($target));
            output.save(&mut File::create(&filename).unwrap(), ImageFormat::PNG).unwrap();
            panic!("{} Saved to: {}", msg, filename);
        }
    };
}

#[test]
fn carver_width_minus_five_test() {
    test_carve!(WIDTH_MINUS_FIVE, -5, 0);
}

#[test]
fn carver_width_plus_five_test() {
    test_carve!(WIDTH_PLUS_FIVE, 5, 0);
}

#[test]
fn carver_height_minus_five_test() {
    test_carve!(HEIGHT_MINUS_FIVE, 0, -5);
}

#[test]
fn carver_height_plus_five_test() {
    test_carve!(HEIGHT_PLUS_FIVE, 0, 5);
}

#[test]
fn carver_both_minus_five_test() {
    test_carve!(BOTH_MINUS_FIVE, -5, -5);
}

#[test]
fn carver_both_plus_five_test() {
    test_carve!(BOTH_PLUS_FIVE, 5, 5);
}

static INPUT: &'static [u8; 12022] = include_bytes!("images/input.png");
static WIDTH_MINUS_FIVE: &'static [u8; 13386] = include_bytes!("images/out-width-minus-five.png");
static WIDTH_PLUS_FIVE: &'static [u8; 14554] = include_bytes!("images/out-width-plus-five.png");
static HEIGHT_MINUS_FIVE: &'static [u8; 13371] = include_bytes!("images/out-height-minus-five.png");
static HEIGHT_PLUS_FIVE: &'static [u8; 14685] = include_bytes!("images/out-height-plus-five.png");
static BOTH_MINUS_FIVE: &'static [u8; 12759] = include_bytes!("images/out-both-minus-five.png");
static BOTH_PLUS_FIVE: &'static [u8; 15226] = include_bytes!("images/out-both-plus-five.png");

fn load(bytes: &[u8]) -> DynamicImage {
    image::load_from_memory(bytes).expect("loaded test image")
}

fn compare_images(target: &DynamicImage, image: &DynamicImage) -> Result<(), &'static str> {
    assert_eq!(target.width(), image.width());
    assert_eq!(target.height(), image.height());

    for (x, y, pixel) in target.pixels() {
        if pixel != image.get_pixel(x, y) {
            return Err("Images do not match!");
        }
    }

    Ok(())
}
