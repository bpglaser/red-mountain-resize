use image::{DynamicImage, GenericImage, Rgba};

use config::Orientation;
use grid::Grid;

pub type Pixel = Rgba<u8>;

struct PixelEnergyPoint {
    pixel: Pixel,
    energy: usize,
    // todo inherited energy
}

pub struct Carver {
    image: DynamicImage, // todo delete? might not be needed
    grid: Grid<PixelEnergyPoint>,
}

impl Carver {
    pub fn new(image: DynamicImage) -> Self {
        let peps = get_pep_grid(&image);
        let grid = Grid::new(peps);
        Self { image, grid }
    }

    pub fn resize(&mut self, distance: isize, orientation: Orientation) -> DynamicImage {
        match orientation {
            Orientation::Horizontal => {}
            Orientation::Vertical => self.grid.rotate(),
        }
        for _ in 0..distance {
            self.resize_once()
        }
        unimplemented!()
    }

    fn resize_once(&mut self) {
        unimplemented!()
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
        columns.push(row);
    }
    columns
}
