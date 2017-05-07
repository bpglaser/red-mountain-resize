use image;
use image::{DynamicImage, GenericImage, Rgba};

use BoxResult;
use Config;
use Grid;

pub type Pixel = Rgba<u8>;

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
        let peps = get_pep_grid(&image);
        let grid = Grid::new(peps);
        Self { image, grid }
    }
}

fn get_pep_grid(image: &DynamicImage) -> Vec<Vec<PixelEnergyPoint>> {
    let (width, height) = image.dimensions();
    let mut columns = vec![];
    for y in 0..height {
        let mut row = vec![];
        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            let pep = PixelEnergyPoint { pixel, energy: 0 };
            row.push(pep);
        }
    }
    columns
}
