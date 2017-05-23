use image::{DynamicImage, GenericImage, Pixel, Rgba};
use num_traits::ToPrimitive;

use config::{Mode, Orientation};
use grid::Grid;

#[derive(Clone)]
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

    pub fn resize(&mut self,
                  distance: usize,
                  orientation: Orientation,
                  mode: Mode)
                  -> DynamicImage {
        match orientation {
            Orientation::Horizontal => self.resize_distance(distance, mode),
            Orientation::Vertical => {
                self.grid.rotate();
                self.resize_distance(distance, mode);
                self.grid.rotate();
            }
        }
        self.rebuild_image()
    }

    fn resize_distance(&mut self, distance: usize, mode: Mode) {
        for _ in 0..distance {
            self.calculate_energy();
            let path = self.find_path();
            match mode {
                Mode::Grow => self.add_path(path),
                Mode::Shrink => self.remove_path(path),
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
        let min_parent_path_cost = self.get_min_parent_path_cost(x, y).unwrap_or(0);
        let energy = self.grid.get(x, y).energy;
        self.grid.get_mut(x, y).path_cost = min_parent_path_cost + energy;
    }

    fn get_min_parent_path_cost(&self, x: usize, y: usize) -> Option<usize> {
        self.grid
            .get_parents(x, y)
            .iter()
            .map(|&(_, _, pep)| pep.path_cost)
            .min()
    }

    fn find_path(&self) -> Vec<(usize, usize)> {
        let mut path = vec![self.get_path_start()];
        loop {
            let &(x, y) = path.last().unwrap();
            match self.get_parent_with_min_path_cost(x, y) {
                None => return path,
                Some(parent) => path.push(parent),
            }
        }
    }

    fn get_path_start(&self) -> (usize, usize) {
        let y = self.grid.height() - 1;
        let (x, _) = self.grid
            .get_row(y)
            .into_iter()
            .enumerate()
            .min_by_key(|&(_, pep)| pep.path_cost)
            .expect("Bottom row should never be empty");
        (x, y)
    }

    fn get_parent_with_min_path_cost(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        self.grid
            .get_parents(x, y)
            .into_iter()
            .min_by_key(|&(_, _, pep)| pep.path_cost)
            .map(|(x, y, _)| (x, y))
    }

    fn add_path(&mut self, points: Vec<(usize, usize)>) {
        self.grid.add_last_column();
        for (x, y) in points {
            self.grid.shift_row_right_from_point(x, y);
        }
    }

    fn remove_path(&mut self, points: Vec<(usize, usize)>) {
        for (x, y) in points {
            self.grid.shift_row_left_from_point(x, y);
        }
        self.grid.remove_last_column();
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
