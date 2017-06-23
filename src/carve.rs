use std::collections::BinaryHeap;

use image::{DynamicImage, GenericImage};

use energy::PixelEnergyPoint;
use grid::{Grid, Token};
use point::Point;

#[derive(Clone)]
pub struct Carver {
    grid: Grid<PixelEnergyPoint>,
    removed_points: Vec<(usize, usize)>,
    dirty_points: Vec<Token>,
    path: Vec<(usize, usize)>,
}

impl Carver {
    pub fn new(image: &DynamicImage) -> Self {
        let grid = image.into();
        Self {
            grid,
            removed_points: vec![],
            dirty_points: vec![],
            path: vec![],
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) -> DynamicImage {
        let initial_width = self.grid.width();
        let initial_height = self.grid.height();

        self.calculate_all_pixel_energy();
        let mut width_changed = false;

        if width > initial_width {
            self.grow_distance(width - initial_width);
            width_changed = true;
        } else if width < initial_width {
            self.shrink_distance(initial_width - width);
            width_changed = true;
        }

        if width_changed && height != initial_height {
            self.calculate_all_pixel_energy();
        }

        if height > initial_height {
            self.grid.rotate();
            self.grow_distance(height - initial_height);
            self.grid.rotate();
        } else if height < initial_height {
            self.grid.rotate();
            self.shrink_distance(initial_height - height);
            self.grid.rotate();
        }

        self.rebuild_image()
    }

    pub fn get_removed_points(self) -> Vec<(usize, usize)> {
        self.removed_points
    }

    fn shrink_distance(&mut self, distance: usize) {
        for _ in 0..distance {
            self.calculate_energy();
            let (start_x, start_y) = self.get_path_start();
            self.find_path(start_x, start_y);
            self.remove_path();
        }
    }

    fn grow_distance(&mut self, distance: usize) {
        let points = self.get_points_removed_by_shrink(distance);

        for _ in 0..distance {
            self.grid.add_last_column();
        }

        for (x, y) in points {
            let pep = {
                let left = self.grid.get(x, y);
                let right = self.grid.get(x + 1, y);
                left.average(&right)
            };
            self.add_point(x, y, pep)
        }
    }

    fn calculate_energy(&mut self) {
        let mut dirty_points: BinaryHeap<Point> = self.dirty_points
            .iter()
            .filter_map(|token| self.grid.downgrade(token))
            .map(From::from)
            .collect();
        self.dirty_points.clear();

        for &Point { x, y } in &dirty_points {
            self.calculate_pixel_energy(x, y);
        }

        while let Some(Point { x, y }) = dirty_points.pop() {
            self.calculate_path_cost(x, y);
        }
    }

    fn calculate_all_pixel_energy(&mut self) {
        for y in 0..self.grid.height() {
            for x in 0..self.grid.width() {
                self.calculate_pixel_energy(x, y);
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
        let min_parent_path_cost = self.get_min_parent_path_cost(x, y);
        let energy = self.grid.get(x, y).energy;
        self.grid.get_mut(x, y).path_cost = min_parent_path_cost + energy;
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

    fn find_path(&mut self, start_x: usize, start_y: usize) {
        self.path.clear();
        self.path.push((start_x, start_y));
        loop {
            let &(x, y) = self.path.last().unwrap();
            match self.get_parent_with_min_path_cost(x, y) {
                None => break,
                Some(parent) => self.path.push(parent),
            }
        }
    }

    fn get_points_removed_by_shrink(&self, distance: usize) -> Vec<(usize, usize)> {
        let mut shrinker = self.clone();

        shrinker.removed_points.clear();
        shrinker.reset_positions();

        shrinker.shrink_distance(distance);
        let mut points = shrinker.get_removed_points();

        // Reverse sort by x values
        points.sort_by(|a, b| b.0.cmp(&a.0));

        points
    }

    fn reset_positions(&mut self) {
        for (x, y, pep) in self.grid.coord_iter_mut() {
            pep.original_position = (x, y);
        }
    }

    fn get_min_parent_path_cost(&self, x: usize, y: usize) -> u32 {
        self.grid
            .get_parents(x, y)
            .into_iter()
            .filter_map(|opt| opt.map(|pep| pep.path_cost))
            .min()
            .unwrap_or(0)
    }

    fn get_parent_with_min_path_cost(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        self.grid
            .get_parents_indexed(x, y)
            .into_iter()
            .min_by_key(|&(_, _, pep)| pep.path_cost)
            .map(|(x, y, _)| (x, y))
    }

    fn add_point(&mut self, x: usize, y: usize, pep: PixelEnergyPoint) {
        self.removed_points
            .push(self.grid.get(x, y).original_position);
        self.grid.shift_row_right_from_point(x, y);
        *self.grid.get_mut(x + 1, y) = pep;
    }

    fn remove_path(&mut self) {
        for &(x, y) in &self.path {
            let adjacent_tokens = self.grid.make_adjacent_tokens(x, y);
            self.dirty_points.extend_from_slice(&adjacent_tokens);

            let original_position = self.grid.get(x, y).original_position;
            self.removed_points.push(original_position);
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

    #[cfg(test)]
    fn get_pixel_energy(&self) -> Vec<Vec<u32>> {
        let mut grid = vec![];
        for y in 0..self.grid.height() {
            let mut row = vec![];
            for x in 0..self.grid.width() {
                row.push(self.grid.get(x, y).energy);
            }
            grid.push(row);
        }
        grid
    }

    #[cfg(test)]
    fn get_path_energy(&self) -> Vec<Vec<u32>> {
        let mut grid = vec![];
        for y in 0..self.grid.height() {
            let mut row = vec![];
            for x in 0..self.grid.width() {
                row.push(self.grid.get(x, y).path_cost);
            }
            grid.push(row);
        }
        grid
    }
}

#[cfg(test)]
mod tests {
    use image;
    use super::Carver;

    macro_rules! setup_carver {
        ( $bytes:expr ) => {
            {
                let input = image::load_from_memory($bytes).unwrap();
                let mut carver = Carver::new(&input);
                carver.calculate_all_pixel_energy();
                carver.calculate_energy();
                carver
            }
        };
    }

    #[test]
    fn carver_small_pixel_energy_test() {
        let carver = setup_carver!(SMALL);
        let pixel_energy = carver.get_pixel_energy();
        assert_eq!(get_small_pixel_energy(), pixel_energy);
    }

    #[test]
    fn carver_small_path_energy_test() {
        let carver = setup_carver!(SMALL);
        let path_energy = carver.get_path_energy();
        assert_eq!(get_small_path_energy(), path_energy);
    }

    #[test]
    fn carver_small_get_path_start_test() {
        let carver = setup_carver!(SMALL);
        assert_eq!((0, 3), carver.get_path_start());
    }

    #[test]
    fn carver_small_find_path_test() {
        let mut carver = setup_carver!(SMALL);
        let (x, y) = carver.get_path_start();
        carver.find_path(x, y);
        assert_eq!(get_small_path(), carver.path);
    }

    #[test]
    fn carver_medium_pixel_energy_test() {
        let carver = setup_carver!(MEDIUM);
        let pixel_energy = carver.get_pixel_energy();
        assert_eq!(get_medium_pixel_energy(), pixel_energy);
    }

    #[test]
    fn carver_medium_path_energy_test() {
        let carver = setup_carver!(MEDIUM);
        let path_energy = carver.get_path_energy();
        assert_eq!(get_medium_path_energy(), path_energy);
    }

    #[test]
    fn carver_medium_get_path_start_test() {
        let carver = setup_carver!(MEDIUM);
        assert_eq!((2, 4), carver.get_path_start());
    }

    #[test]
    fn carver_medium_find_path_test() {
        let mut carver = setup_carver!(MEDIUM);
        let (x, y) = carver.get_path_start();
        carver.find_path(x, y);
        assert_eq!(get_medium_path(), carver.path);
    }

    static SMALL: &'static [u8; 173] = include_bytes!("../tests/images/small_energy.png");
    static MEDIUM: &'static [u8; 244] = include_bytes!("../tests/images/medium_energy.png");

    fn get_small_pixel_energy() -> Vec<Vec<u32>> {
        vec![vec![20808, 52020, 20808],
             vec![20808, 52225, 21220],
             vec![20809, 52024, 20809],
             vec![20808, 52225, 21220]]
    }

    fn get_small_path_energy() -> Vec<Vec<u32>> {
        vec![vec![20808, 52020, 20808],
             vec![41616, 73033, 42028],
             vec![62425, 93640, 62837],
             vec![83233, 114650, 84057]]
    }

    fn get_small_path() -> Vec<(usize, usize)> {
        vec![(0, 3), (0, 2), (0, 1), (0, 0)]
    }

    fn get_medium_pixel_energy() -> Vec<Vec<u32>> {
        vec![vec![57685, 50893, 91370, 25418, 33055, 37246],
             vec![15421, 56334, 22808, 54796, 11641, 25496],
             vec![12344, 19236, 52030, 17708, 44735, 20663],
             vec![17074, 23678, 30279, 80663, 37831, 45595],
             vec![32337, 30796, 4909, 73334, 40613, 36556]]
    }

    fn get_medium_path_energy() -> Vec<Vec<u32>> {
        vec![vec![57685, 50893, 91370, 25418, 33055, 37246],
             vec![66314, 107227, 48226, 80214, 37059, 58551],
             vec![78658, 67462, 100256, 54767, 81794, 57722],
             vec![84536, 91140, 85046, 135430, 92598, 103317],
             vec![116873, 115332, 89955, 158380, 133211, 129154]]
    }

    fn get_medium_path() -> Vec<(usize, usize)> {
        vec![(2, 4), (2, 3), (3, 2), (4, 1), (3, 0)]
    }
}
