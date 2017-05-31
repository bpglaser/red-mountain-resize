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

fn run(mut config: Config) -> BoxResult<()> {
    let image = image::open(&config.input_path)?;
    let mut carver = Carver::new(&image);

    let scaled_image = scale_image(&mut carver, &config);
    save_image_to_path(&scaled_image, config.get_output_path())?;

    if let Some(debug_path) = config.debug_path {
        let debug_image = create_debug_image(&image, &carver.get_removed_points());
        save_image_to_path(&debug_image, debug_path)?;
    }

    Ok(())
}

fn scale_image(carver: &mut Carver, config: &Config) -> DynamicImage {
    unimplemented!() //TODO determine what carve operations to preform
}

fn save_image_to_path<P: AsRef<Path>>(image: &DynamicImage, path: P) -> BoxResult<()> {
    let mut file = File::create(path)?;
    image.save(&mut file, ImageFormat::PNG)?;
    Ok(())
}
