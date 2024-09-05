use std::borrow::Cow;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use anyhow::{bail, Ok, Result};
use clap::Parser;
use image::{DynamicImage, GenericImageView};

use rmr::carve::Carver;
use rmr::change::Change;
use rmr::config::Config;
use rmr::debug::create_debug_image;

fn main() -> Result<()> {
    pretty_env_logger::init();
    let cfg = Config::parse();
    run(cfg)?;
    Ok(())
}

fn run(mut config: Config) -> Result<()> {
    log::info!("loading: {:?}", &config.input_path);
    let mut image = image::open(&config.input_path)?;
    let carver = Rc::new(RefCell::new(Carver::new(&image)));

    let dimensions = get_target_dimensions(&image, &config)?;
    log::info!(
        "target dimensions [{}]",
        dimensions
            .clone()
            .iter()
            .map(|dim| format!("{:?}", dim))
            .collect::<Vec<_>>()
            .join(", ")
    );
    for (width, height) in dimensions.iter().cloned() {
        log::info!("cloning carver");
        let carver = carver.clone();
        log::info!("resizing to: {:?}", (width, height));
        let scaled_image = carver.borrow_mut().resize(width, height);
        log::info!("finished resizing");

        let mut suffix = Cow::Borrowed("");
        if dimensions.len() > 1 {
            suffix = format!("{:?}", (width, height)).into();
        }
        let path = config.get_output_path(&suffix);
        log::info!("saving output image");
        save_image_to_path(&scaled_image, path)?;

        if let Some(debug_path) = &config.debug_path {
            let debug_image = create_debug_image(&mut image, carver.borrow().get_removed_points());
            log::info!("saving debug image");
            save_image_to_path(&debug_image, debug_path)?;
        }
    }

    Ok(())
}

fn get_target_dimensions(image: &DynamicImage, config: &Config) -> Result<Vec<(usize, usize)>> {
    let (width, height) = image.dimensions();
    let dims @ (width, height) = (width as usize, height as usize);

    if let Some(dimensions) = &config.dimensions {
        return collect_both_dimensions(&dimensions.0, &dimensions.1, dims);
    }

    match (&config.width, &config.height) {
        (None, None) => bail!("no resize dimensions specified"),
        (None, Some(height_change)) => Ok(height_change
            .to_absolutes(height)?
            .into_iter()
            .map(|h| (width, h))
            .collect()),
        (Some(width_change), None) => Ok(width_change
            .to_absolutes(width)?
            .into_iter()
            .map(|w| (w, height))
            .collect()),
        (Some(width_change), Some(height_change)) => {
            collect_both_dimensions(width_change, height_change, dims)
        }
    }
}

fn collect_both_dimensions(
    width_change: &Change,
    height_change: &Change,
    (width, height): (usize, usize),
) -> Result<Vec<(usize, usize)>> {
    let mut res = vec![];
    for w in width_change.to_absolutes(width)? {
        for h in height_change.to_absolutes(height)? {
            res.push((w, h));
        }
    }
    Ok(res)
}

fn save_image_to_path<P: AsRef<Path>>(image: &DynamicImage, path: P) -> Result<()> {
    let path = path.as_ref();
    log::info!(
        "saving image of size {:?} to {:?}",
        image.dimensions(),
        path
    );
    image.save(path)?;
    Ok(())
}
