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
    grid: Grid<PixelEnergyPoint>,
}

impl Carver {
    pub fn new(image: DynamicImage) -> Self {
        let peps = get_pep_grid(&image);
        let grid = Grid::new(peps);
        Self { grid }
    }

    pub fn resize(&mut self, distance: isize, orientation: Orientation) -> DynamicImage {
        match orientation {
            Orientation::Horizontal => self.resize_distance(distance),
            Orientation::Vertical => {
                self.grid.rotate();
                self.resize_distance(distance);
                self.grid.rotate();
            }
        }
        self.rebuild_image()
    }

    fn resize_distance(&mut self, distance: isize) {
        for _ in 0..distance {
            self.resize_once()
        }
    }

    fn resize_once(&mut self) {
        unimplemented!()
    }

    fn rebuild_image(&self) -> DynamicImage {
        let mut image = DynamicImage::new_rgba8(self.grid.width() as u32,
                                                self.grid.height() as u32);
        for (x, y, pep) in self.grid.coord_iter() {
            image.put_pixel(x as u32, y as u32, pep.pixel);
        }
        image
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
