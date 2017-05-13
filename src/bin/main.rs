extern crate image;
extern crate seam_carving_resize;

use seam_carving_resize::BoxResult;
use seam_carving_resize::carve::Carver;
use seam_carving_resize::config::{Config, parse_args};

fn main() {
    parse_args().and_then(run).unwrap()
}

fn run(config: Config) -> BoxResult<()> {
    let image = image::open(config.file_path)?;
    let mut carver = Carver::new(image);
    carver.resize(config.distance, config.orientation);
    Ok(())
}
