extern crate image;
extern crate rmr;

use std::fs::File;
use std::path::Path;

use image::{DynamicImage, ImageFormat};

use rmr::BoxResult;
use rmr::config::{Config, Mode, parse_args};
use rmr::grow::Grower;
use rmr::shrink::Shrinker;

fn main() {
    parse_args().and_then(run).unwrap()
}

fn run(config: Config) -> BoxResult<()> {
    let image = image::open(config.file_path)?;

    let scaled_image = match config.mode {
        Mode::Grow => grow(&image, config.distance),
        Mode::Shrink => shrink(&image, config.distance),
    };

    save_image_to_path(&scaled_image, config.save_path)
}

fn grow(image: &DynamicImage, distance: u32) -> DynamicImage {
    let grower = Grower::new(&image);
    grower.create_enlarged_image(distance)
}

fn shrink(image: &DynamicImage, distance: u32) -> DynamicImage {
    let shrinker = Shrinker::new(&image, distance);
    shrinker.create_reduced_image()
}

fn save_image_to_path<P: AsRef<Path>>(image: &DynamicImage, path: P) -> BoxResult<()> {
    let mut file = File::create(path)?;
    image.save(&mut file, ImageFormat::PNG)?;
    Ok(())
}
