extern crate image;
extern crate rmr;

use std::fs::File;
use std::path::Path;
use std::time::Instant;

use image::{DynamicImage, GenericImage, ImageFormat};

use rmr::BoxResult;
use rmr::carve::{Carver, create_debug_image};
use rmr::config::{Config, parse_args};

fn main() {
    parse_args().and_then(run).unwrap()
}

fn run(mut config: Config) -> BoxResult<()> {
    let image = image::open(&config.input_path)?;
    let mut carver = Carver::new(&image);

    let (width, height) = get_target_dimensions(&image, &config);

    let time_start = if config.time {
        Some(Instant::now())
    } else {
        None
    };

    let scaled_image = carver.resize(width as usize, height as usize); // TODO usize -> u32

    if let Some(time_start) = time_start {
        let duration = time_start.elapsed();
        let secs = duration.as_secs();
        let nanos = duration.subsec_nanos();
        println!("Resing image took: {}.{}", secs, nanos);
    }

    save_image_to_path(&scaled_image, config.get_output_path())?;

    if let Some(debug_path) = config.debug_path {
        let debug_image = create_debug_image(&image, &carver.get_removed_points());
        save_image_to_path(&debug_image, debug_path)?;
    }

    Ok(())
}

fn get_target_dimensions(image: &DynamicImage, config: &Config) -> (u32, u32) {
    if let Some(dimensions) = config.dimensions {
        return dimensions;
    }

    let (mut width, mut height) = image.dimensions();

    if let Some(delta_width) = config.width {
        if delta_width >= 0 {
            width += delta_width as u32;
        } else {
            width -= delta_width.abs() as u32;
        }
    }

    if let Some(delta_height) = config.height {
        if delta_height >= 0 {
            height += delta_height as u32;
        } else {
            height -= delta_height.abs() as u32;
        }
    }

    (width, height)
}

fn save_image_to_path<P: AsRef<Path>>(image: &DynamicImage, path: P) -> BoxResult<()> {
    let mut file = File::create(path)?;
    image.save(&mut file, ImageFormat::PNG)?;
    Ok(())
}
