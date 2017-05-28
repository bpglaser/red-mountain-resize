extern crate image;
extern crate rmr;

use std::fs::File;
use std::path::Path;

use image::{DynamicImage, GenericImage, ImageFormat};

use rmr::BoxResult;
use rmr::config::{Config, Mode, parse_args};
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
    unimplemented!()
}

fn shrink(image: &DynamicImage, distance: u32) -> DynamicImage {
    let mut shrinker = Shrinker::new(&image);
    let points = shrinker.calculate_removal_points(distance);
    create_smaller_image(&image, distance, &points)
}

fn create_smaller_image(image: &DynamicImage,
                        distance: u32,
                        points: &[(u32, u32)])
                        -> DynamicImage {

    let (width, height) = image.dimensions();
    let width = width - distance;

    let mut smaller_image = DynamicImage::new_rgba8(width, height);
    let mut new_x = 0;
    let mut new_y = 0;

    for (x, y, pixel) in image.pixels() {
        if points.contains(&(x, y)) {
            continue;
        }

        smaller_image.put_pixel(new_x, new_y, pixel);

        new_x += 1;
        if new_x == width {
            new_x = 0;
            new_y += 1;
        }
    }

    smaller_image
}

fn save_image_to_path<P: AsRef<Path>>(image: &DynamicImage, path: P) -> BoxResult<()> {
    let mut file = File::create(path)?;
    image.save(&mut file, ImageFormat::PNG)?;
    Ok(())
}
