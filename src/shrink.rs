use std::collections::HashSet;

use image::{DynamicImage, GenericImage};

use energy::PixelEnergyPoint;
use grid::Grid;

pub struct Shrinker<'a> {
    source: &'a DynamicImage,
    distance: u32,
    grid: Grid<PixelEnergyPoint>,
    points: Option<HashSet<(u32, u32)>>,
}

impl<'a> Shrinker<'a> {
    pub fn new(source: &'a DynamicImage, distance: u32) -> Self {
        Self {
            source,
            distance,
            grid: source.into(),
            points: None,
        }
    }

    pub fn create_reduced_image(mut self) -> DynamicImage {
        if self.points.is_none() {
            self.calculate_points();
        }

        let (width, height) = self.source.dimensions();
        let width = width - self.distance;
        let mut reduced_image = DynamicImage::new_rgba8(width, height);

        let (mut new_x, mut new_y) = (0, 0);
        for (x, y, pixel) in self.source.pixels() {
            if self.points.as_ref().unwrap().contains(&(x, y)) {
                continue;
            }

            reduced_image.put_pixel(new_x, new_y, pixel);

            new_x += 1;
            if new_x == width {
                new_x = 0;
                new_y += 1;
            }
        }

        reduced_image
    }

    pub fn calculate_points(&mut self) {
        if self.points.is_some() {
            return;
        }
        self.points = Some(HashSet::new());

        for _ in 0..self.distance {
            self.calculate_energy();
            let (start_x, start_y) = self.get_path_start();
            let path = self.find_path(start_x, start_y);
            self.remove_path(path);
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
            let horizontal_square_gradient = left.square_gradient(right);
            let vertical_square_gradient = up.square_gradient(down);
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

    fn find_path(&self, start_x: usize, start_y: usize) -> Vec<(usize, usize)> {
        let mut path = vec![(start_x, start_y)];
        loop {
            let &(x, y) = path.last().unwrap();
            match self.get_parent_with_min_path_cost(x, y) {
                None => return path,
                Some(parent) => path.push(parent),
            }
        }
    }

    fn get_parent_with_min_path_cost(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        self.grid
            .get_parents(x, y)
            .into_iter()
            .min_by_key(|&(_, _, pep)| pep.path_cost)
            .map(|(x, y, _)| (x, y))
    }

    fn remove_path(&mut self, points: Vec<(usize, usize)>) {
        for (x, y) in points {
            self.grid.mark_removed(x, y);
        }
    }
}
