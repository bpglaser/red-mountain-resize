extern crate image;
extern crate seam_carving_resize;

use std::fs::File;
use std::path::Path;

use image::{DynamicImage, ImageFormat};

use seam_carving_resize::BoxResult;
use seam_carving_resize::carve::Carver;
use seam_carving_resize::config::{Config, parse_args};

fn main() {
    parse_args().and_then(run).unwrap()
}

fn run(config: Config) -> BoxResult<()> {
    let image = image::open(config.file_path)?;
    let mut carver = Carver::new(image);
    let scaled_image = carver.resize(config.distance, config.orientation);
    save_to_config_path(scaled_image, config.save_path)
}

fn save_to_config_path<P: AsRef<Path>>(image: DynamicImage, path: P) -> BoxResult<()> {
    let mut file = File::create(path)?;
    image.save(&mut file, ImageFormat::PNG)?;
    Ok(())
}
