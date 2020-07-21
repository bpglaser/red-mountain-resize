use std::path::Path;
use std::time::Instant;

use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

use rmr::carve::Carver;
use rmr::config::{parse_args, Config};
use rmr::BoxResult;

fn main() {
    parse_args().and_then(run).unwrap()
}

fn run(mut config: Config) -> BoxResult<()> {
    let mut image = image::open(&config.input_path)?;
    let mut carver = Carver::new(&image);

    let (width, height) = get_target_dimensions(&image, &config);

    let time_start = if config.time {
        Some(Instant::now())
    } else {
        None
    };

    let scaled_image = carver.resize(width, height);

    if let Some(time_start) = time_start {
        let duration = time_start.elapsed();
        let secs = duration.as_secs();
        let nanos = duration.subsec_nanos();
        println!("Resizing image took: {}.{}", secs, nanos);
    }

    save_image_to_path(&scaled_image, config.get_output_path())?;

    if let Some(debug_path) = config.debug_path {
        let debug_image = create_debug_image(&mut image, &carver.get_removed_points());
        save_image_to_path(&debug_image, debug_path)?;
    }

    Ok(())
}

fn get_target_dimensions(image: &DynamicImage, config: &Config) -> (usize, usize) {
    if let Some(dimensions) = config.dimensions {
        return dimensions;
    }

    let (width, height) = image.dimensions();
    let (mut width, mut height) = (width as usize, height as usize);

    if let Some(delta_width) = config.width {
        if delta_width >= 0 {
            width += delta_width as usize;
        } else {
            width -= delta_width.abs() as usize;
        }
    }

    if let Some(delta_height) = config.height {
        if delta_height >= 0 {
            height += delta_height as usize;
        } else {
            height -= delta_height.abs() as usize;
        }
    }

    (width, height)
}

fn save_image_to_path<P: AsRef<Path>>(image: &DynamicImage, path: P) -> BoxResult<()> {
    image.save(path)?;
    Ok(())
}

fn create_debug_image(image: &mut DynamicImage, points: &[(usize, usize)]) -> DynamicImage {
    let red_pixel = Rgba([255, 0, 0, 255]);
    let mut image = image.clone();
    for &(x, y) in points {
        image.put_pixel(x as u32, y as u32, red_pixel);
    }
    image
}
