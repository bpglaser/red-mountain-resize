use std::mem::swap;

use image::{DynamicImage, GenericImage};

use energy::PixelEnergyPoint;

#[derive(Clone)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    points: Vec<T>,
    rotated: bool,
}

impl<T> Grid<T> {
    pub fn new(points: Vec<T>, width: usize, height: usize) -> Self {
        let rotated = false;
        Self {
            width,
            height,
            points,
            rotated,
        }
    }

    pub fn height(&self) -> usize {
        if !self.rotated {
            self.height
        } else {
            self.width
        }
    }

    pub fn width(&self) -> usize {
        if !self.rotated {
            self.width
        } else {
            self.height
        }
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        let i = self.get_point_index(x, y);
        &self.points[i]
    }

    pub fn get_adjacent(&self, x: usize, y: usize) -> (&T, &T, &T, &T) {
        let left = if x == 0 {
            self.get(self.width() - 1, y)
        } else {
            self.get(x - 1, y)
        };

        let right = if x == self.width() - 1 {
            self.get(0, y)
        } else {
            self.get(x + 1, y)
        };

        let up = if y == 0 {
            self.get(x, self.height() - 1)
        } else {
            self.get(x, y - 1)
        };

        let down = if y == self.height() - 1 {
            self.get(x, 0)
        } else {
            self.get(x, y + 1)
        };

        (left, right, up, down)
    }

    pub fn get_parents(&self, x: usize, y: usize) -> Vec<(usize, usize, &T)> {
        let mut parents = vec![];

        if y > 0 {
            if x > 0 {
                parents.push((x - 1, y - 1, self.get(x - 1, y - 1)));
            }

            parents.push((x, y - 1, self.get(x, y - 1)));

            if x < self.width() - 1 {
                parents.push((x + 1, y - 1, self.get(x + 1, y - 1)));
            }
        }

        parents
    }

    pub fn get_row(&self, y: usize) -> Vec<&T> {
        let mut row = vec![];
        for x in 0..self.width() {
            row.push(self.get(x, y));
        }
        row
    }

    pub fn get_row_with_coords(&self, y: usize) -> Vec<(usize, usize, &T)> {
        self.get_row(y)
            .into_iter()
            .enumerate()
            .map(|(x, pep)| (x, y, pep))
            .collect()
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        let i = self.get_point_index(x, y);
        &mut self.points[i]
    }

    pub fn rotate(&mut self) {
        self.rotated = !self.rotated;
        swap(&mut self.width, &mut self.height);
    }

    pub fn is_rotated(&self) -> bool {
        self.rotated
    }

    pub fn coord_iter(&self) -> PointIter<T> {
        PointIter {
            x: 0,
            y: 0,
            grid: &self,
        }
    }

    pub fn remove_last_column(&mut self) {
        let x = self.width - 1;
        for y in (0..self.height - 1).rev() {
            let i = self.get_point_index(x, y);
            self.points.remove(i);
        }
        self.width -= 1;
    }

    fn get_point_index(&self, x: usize, y: usize) -> usize {
        if !self.rotated {
            x + y * self.width
        } else {
            y + x * self.height
        }
    }
}

impl<T: Clone> Grid<T> {
    pub fn shift_row_left_from_point(&mut self, x: usize, y: usize) {
        for x in x..(self.width() - 1) {
            *self.get_mut(x, y) = self.get(x + 1, y).clone();
        }
    }

    pub fn shift_row_right_from_point(&mut self, x: usize, y: usize) {
        for x in (x + 1..self.width()).rev() {
            *self.get_mut(x, y) = self.get(x - 1, y).clone();
        }
    }

    pub fn add_last_column(&mut self) {
        unimplemented!()
    }
}

impl<'a> From<&'a DynamicImage> for Grid<PixelEnergyPoint> {
    fn from(image: &'a DynamicImage) -> Self {
        let points = image.pixels().map(|(_, _, pixel)| pixel.into()).collect();
        let (width, height) = image.dimensions();
        Grid::new(points, width as usize, height as usize)
    }
}

pub struct PointIter<'a, T: 'a> {
    x: usize,
    y: usize,
    grid: &'a Grid<T>,
}

impl<'a, T> Iterator for PointIter<'a, T> {
    type Item = (usize, usize, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.grid.height() {
            return None;
        }

        let x = self.x;
        let y = self.y;
        let val = self.grid.get(x, y);

        self.x += 1;
        if self.x >= self.grid.width() {
            self.x = 0;
            self.y += 1;
        }

        Some((x, y, val))
    }
}
