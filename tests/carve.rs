use std::path::PathBuf;

use image::{DynamicImage, GenericImageView, RgbImage};

use rmr::carve::Carver;

static INPUT: &[u8] = include_bytes!("images/input.png");
static WIDTH_MINUS_FIVE: &[u8] = include_bytes!("images/out-width-minus-five.png");
static WIDTH_PLUS_FIVE: &[u8] = include_bytes!("images/out-width-plus-five.png");
static HEIGHT_MINUS_FIVE: &[u8] = include_bytes!("images/out-height-minus-five.png");
static HEIGHT_PLUS_FIVE: &[u8] = include_bytes!("images/out-height-plus-five.png");
static BOTH_MINUS_FIVE: &[u8] = include_bytes!("images/out-both-minus-five.png");
static BOTH_PLUS_FIVE: &[u8] = include_bytes!("images/out-both-plus-five.png");

macro_rules! test_carve {
    ( $target:expr, $dw:expr, $dh:expr ) => {
        let input = load(INPUT);

        let (width, height) = input.dimensions();
        let target_width = (width as isize + $dw) as usize;
        let target_height = (height as isize + $dh) as usize;

        let mut carver = Carver::new(&input);
        let output = carver.resize(target_width, target_height);

        let target = load($target);
        if let Err(diff) = compare_images(&target, &output) {
            let filename = format!("{}_DIFF.png", stringify!($target));
            let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(filename);
            diff.save(&path).unwrap();
            panic!("Saved diff to: {:?}", path);
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

fn load(bytes: &[u8]) -> DynamicImage {
    image::load_from_memory(bytes).expect("loaded test image")
}

fn compare_images(target: &DynamicImage, image: &DynamicImage) -> Result<(), DynamicImage> {
    assert_eq!(target.width(), image.width());
    assert_eq!(target.height(), image.height());
    let (width, height) = target.dimensions();

    let mut diff = None;

    for (x, y, pixel) in target.pixels() {
        if pixel != image.get_pixel(x, y) {
            let diff = diff.get_or_insert_with(|| RgbImage::new(width, height));
            diff.put_pixel(x, y, [255, 0, 0].into());
        }
    }

    match diff {
        Some(img) => Err(img.into()),
        None => Ok(()),
    }
}
