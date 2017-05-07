use image;
use image::{DynamicImage, Rgba};

use ArgConfig;
use BoxResult;
use Grid;

pub fn run(config: ArgConfig) -> BoxResult<()> {
    let image = image::open(&config.file_path)?;
    let mut carver = Carver::new(image);
    Ok(())
}

type Pixel = Rgba<u8>;

struct PixelEnergyPoint {
    pixel: Pixel,
    energy: usize,
    // todo inherited energy
}

struct Carver {
    image: DynamicImage,
    grid: Grid<PixelEnergyPoint>,
}

impl Carver {
    fn new(image: DynamicImage) -> Self {
        unimplemented!()
    }
}
