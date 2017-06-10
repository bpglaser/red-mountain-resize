use image::{DynamicImage, GenericImage, Rgba};

use energy::PixelEnergyPoint;
use grid::Grid;

#[derive(Clone)]
pub struct Carver {
    grid: Grid<PixelEnergyPoint>,
    removed_points: Vec<(usize, usize)>,
}

impl Carver {
    pub fn new(image: &DynamicImage) -> Self {
        let grid = image.into();
        Self {
            grid,
            removed_points: vec![],
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) -> DynamicImage {
        let initial_width = self.grid.width();
        let initial_height = self.grid.height();

        if width > initial_width {
            self.grow_distance(width - initial_width);
        } else if width < initial_width {
            self.shrink_distance(initial_width - width);
        }

        if height > initial_height {
            self.rotate();
            self.grow_distance(height - initial_height);
            self.rotate();
        } else if height < initial_height {
            self.rotate();
            self.shrink_distance(initial_height - height);
            self.rotate();
        }

        self.rebuild_image()
    }

    pub fn get_removed_points(&self) -> Vec<(usize, usize)> {
        self.removed_points.clone()
    }

    pub fn calculate_energy(&mut self) {
        for y in 0..self.grid.height() {
            for x in 0..self.grid.width() {
                self.calculate_pixel_energy(x, y);
                self.calculate_path_cost(x, y);
            }
        }
    }

    pub fn get_pixel_energy(&self) -> Vec<Vec<u32>> {
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

    pub fn get_path_energy(&self) -> Vec<Vec<u32>> {
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

    pub fn get_path_start(&self) -> (usize, usize) {
        let y = self.grid.height() - 1;
        let (x, _) = self.grid
            .get_row(y)
            .into_iter()
            .enumerate()
            .min_by_key(|&(_, pep)| pep.path_cost)
            .expect("Bottom row should never be empty");
        (x, y)
    }

    fn grow_distance(&mut self, distance: usize) {
        let points = self.get_points_removed_by_shrink(distance);

        for _ in 0..distance {
            self.grid.add_last_column();
        }

        for (x, y) in points {
            let left = self.grid.get(x, y).pixel;
            let pixel = self.average_pixel_from_neighbors(x, y, left);
            self.add_point(x, y, pixel)
        }
    }

    fn get_points_removed_by_shrink(&self, distance: usize) -> Vec<(usize, usize)> {
        let mut shrinker = self.clone();
        shrinker.removed_points.clear();
        shrinker.shrink_distance(distance);
        let mut points = shrinker.get_removed_points();

        // Reverse sort by x values
        points.sort_by(|a, b| b.0.cmp(&a.0));

        points
    }

    fn shrink_distance(&mut self, distance: usize) {
        for _ in 0..distance {
            self.calculate_energy();
            let (start_x, start_y) = self.get_path_start();
            let path = self.find_path(start_x, start_y);
            self.remove_path(path);
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

    fn get_min_parent_path_cost(&self, x: usize, y: usize) -> u32 {
        self.grid
            .get_parents(x, y)
            .into_iter()
            .filter_map(|opt| opt.map(|pep| pep.path_cost))
            .min()
            .unwrap_or(0)
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
            .get_parents_indexed(x, y)
            .into_iter()
            .min_by_key(|&(_, _, pep)| pep.path_cost)
            .map(|(x, y, _)| (x, y))
    }

    fn add_point(&mut self, x: usize, y: usize, pixel: Rgba<u8>) {
        self.removed_points.push((x, y));
        self.grid.shift_row_right_from_point(x, y);
        *self.grid.get_mut(x + 1, y) = pixel.into();
    }

    fn average_pixel_from_neighbors(&self, x: usize, y: usize, left: Rgba<u8>) -> Rgba<u8> {
        let right = self.grid.get(x + 1, y).pixel;
        let data = average_pixels(&left.data, &right.data);
        Rgba { data }
    }

    fn remove_path(&mut self, points: Vec<(usize, usize)>) {
        for (x, y) in points {
            self.removed_points.push((x, y));
            self.grid.shift_row_left_from_point(x, y);
        }
        self.grid.remove_last_column();
    }

    fn rotate(&mut self) {
        self.grid.rotate();
        self.rotate_removed_points();
    }

    fn rotate_removed_points(&mut self) {
        for point in self.removed_points.iter_mut() {
            *point = (point.1, point.0);
        }
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

fn average_pixels(pixel1: &[u8; 4], pixel2: &[u8; 4]) -> [u8; 4] {
    [((pixel1[0] as u16 + pixel2[0] as u16) / 2) as u8,
     ((pixel1[1] as u16 + pixel2[1] as u16) / 2) as u8,
     ((pixel1[2] as u16 + pixel2[2] as u16) / 2) as u8,
     ((pixel1[3] as u16 + pixel2[3] as u16) / 2) as u8]
}

pub fn create_debug_image(image: &DynamicImage, points: &[(usize, usize)]) -> DynamicImage {
    let red_pixel = Rgba { data: [255, 0, 0, 255] };
    let mut image = image.clone();
    for &(x, y) in points {
        image.put_pixel(x as u32, y as u32, red_pixel);
    }
    image
}
