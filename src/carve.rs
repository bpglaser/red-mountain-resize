use image::{DynamicImage, GenericImage, Pixel, Rgba};
use num_traits::ToPrimitive;

use config::Orientation;
use grid::Grid;

struct PixelEnergyPoint {
    pixel: Rgba<u8>,
    energy: usize,
    path_cost: usize,
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
        let shrink_image = distance < 0;
        let distance = distance as usize;

        for _ in 0..distance {
            self.calculate_energy();
            let path = self.find_path();
            if shrink_image {
                self.remove_path(path);
            } else {
                self.add_path(path);
            }
        }
    }

    fn calculate_energy(&mut self) {
        for y in 0..self.grid.height() {
            for x in 0..self.grid.width() {
                self.calculate_pixel_energy(x, y);
                self.calculate_path_cost(x, y);
            }
        }
    }

    fn calculate_pixel_energy(&mut self, x: usize, y: usize) {
        let energy = {
            let (left, right, up, down) = self.grid.get_adjacent(x, y);
            let horizontal_square_gradient = square_gradient(left, right);
            let vertical_square_gradient = square_gradient(up, down);
            horizontal_square_gradient + vertical_square_gradient
        };
        self.grid.get_mut(x, y).energy = energy;
    }

    fn calculate_path_cost(&mut self, x: usize, y: usize) {
        let min_parent_energy = {
            let parents = self.grid.get_parents(x, y);
            parents.iter().map(|pep| pep.energy).min().unwrap_or(0)
        };
        self.grid.get_mut(x, y).path_cost = min_parent_energy;
    }

    fn find_path(&self) -> Vec<(isize, isize)> {
        unimplemented!()
    }

    fn add_path(&mut self, points: Vec<(isize, isize)>) {
        unimplemented!()
    }

    fn remove_path(&mut self, points: Vec<(isize, isize)>) {
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
            let pep = PixelEnergyPoint {
                pixel,
                energy: 0,
                path_cost: 0,
            };
            row.push(pep);
        }
        columns.push(row);
    }
    columns
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
