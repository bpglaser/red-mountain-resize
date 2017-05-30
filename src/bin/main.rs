extern crate image;
extern crate rmr;

use std::fs::File;
use std::path::Path;

use image::{DynamicImage, ImageFormat};

use rmr::BoxResult;
use rmr::carve::{Carver, create_debug_image};
use rmr::config::{Config, parse_args};

fn main() {
    parse_args().and_then(run).unwrap()
}

fn run(config: Config) -> BoxResult<()> {
    let image = image::open(config.file_path)?;
    let mut carver = Carver::new(&image);

    let scaled_image = carver.resize(config.distance, config.orientation, config.mode);
    save_image_to_path(&scaled_image, config.save_path)?;

    if let Some(debug_image_path) = config.debug_image_path {
        let debug_image = create_debug_image(&image, &carver.get_removed_points());
        save_image_to_path(&debug_image, debug_image_path)?;
    }
    Ok(())
}

fn save_image_to_path<P: AsRef<Path>>(image: &DynamicImage, path: P) -> BoxResult<()> {
    let mut file = File::create(path)?;
    image.save(&mut file, ImageFormat::PNG)?;
    Ok(())
}
