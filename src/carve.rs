use image::{DynamicImage, GenericImage, Pixel, Rgba};
use num_traits::ToPrimitive;

use config::Orientation;
use grid::Grid;

struct PixelEnergyPoint {
    pixel: Rgba<u8>,
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

fn calculate_pixel_energy(grid: &Grid<PixelEnergyPoint>, x: u32, y: u32) -> usize {
    let (left, right, up, down) = grid.get_adjacent(x as isize, y as isize);
    let horizontal_square_gradient = square_gradient(left, right);
    let vertical_square_gradient = square_gradient(up, down);
    horizontal_square_gradient + vertical_square_gradient
}

fn square_gradient(pep1: &PixelEnergyPoint, pep2: &PixelEnergyPoint) -> usize {
    let pixel1 = pep1.pixel;
    let pixel2 = pep2.pixel;

    let pixel1_channels = pixel1.channels();
    let pixel2_channels = pixel2.channels();

    let mut sum = 0;
    for i in 0..pixel1_channels.len() {
        let a = pixel1_channels[i]
            .to_isize()
            .expect("Unable to convert value");
        let b = pixel2_channels[i]
            .to_isize()
            .expect("Unable to convert value");
        sum += (a - b).abs().pow(2); // Squared abs difference
    }
    sum as usize
}
