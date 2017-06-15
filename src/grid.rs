use std::cell::Cell;
use std::rc::{Rc, Weak};

use image::{DynamicImage, GenericImage};

use energy::PixelEnergyPoint;

type StrongPosition = Rc<Cell<(usize, usize)>>;
type WeakPosition = Weak<Cell<(usize, usize)>>;

#[derive(Clone)]
pub struct Token {
    position: WeakPosition,
}

impl Token {
    fn try_get(self) -> Option<(usize, usize)> {
        self.position.upgrade().map(|p| p.get())
    }
}

#[derive(Debug)]
pub struct Grid<T> {
    points: Vec<Vec<(T, Option<StrongPosition>)>>,
    rotated: bool,
}

impl<T> Grid<T> {
    pub fn new(points: Vec<Vec<T>>) -> Self {
        let points = Grid::convert_container(points);
        let rotated = false;
        Self { points, rotated }
    }

    pub fn height(&self) -> usize {
        if !self.rotated {
            self.points.len()
        } else {
            self.points[0].len()
        }
    }

    pub fn width(&self) -> usize {
        if !self.rotated {
            self.points[0].len()
        } else {
            self.points.len()
        }
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        if !self.rotated {
            &self.points[y][x].0
        } else {
            &self.points[x][y].0
        }
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

    pub fn apply_adjacent<F>(&mut self, x: usize, y: usize, func: F)
        where F: Fn(&mut T)
    {
        let x_left = if x == 0 { self.width() - 1 } else { x - 1 };
        func(self.get_mut(x_left, y));

        let x_right = if x == self.width() - 1 { 0 } else { x + 1 };
        func(self.get_mut(x_right, y));

        let y_up = if y == 0 { self.height() - 1 } else { y - 1 };
        func(self.get_mut(x, y_up));

        let y_down = if y == self.height() - 1 { 0 } else { y + 1 };
        func(self.get_mut(x, y_down));
    }

    pub fn get_parents(&self, x: usize, y: usize) -> [Option<&T>; 3] {
        let mut parents = [None; 3];

        if y > 0 {
            let y = y - 1;

            if x > 0 {
                parents[0] = Some(self.get(x - 1, y));
            }

            parents[1] = Some(self.get(x, y));

            if x < self.width() - 1 {
                parents[2] = Some(self.get(x + 1, y));
            }
        }

        parents
    }

    pub fn get_parents_indexed(&self, x: usize, y: usize) -> Vec<(usize, usize, &T)> {
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
        if !self.rotated {
            &mut self.points[y][x].0
        } else {
            &mut self.points[x][y].0
        }
    }

    pub fn rotate(&mut self) {
        self.rotated = !self.rotated;
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
        let expect_msg = "Attempted to remove column from empty grid";
        if self.rotated {
            self.points.pop().expect(expect_msg);
        } else {
            for mut row in self.points.iter_mut() {
                row.pop().expect(expect_msg);
            }
        }
    }

    pub fn make_token(&mut self, mut x: usize, mut y: usize) -> Token {
        if self.is_rotated() {
            let tmp = x;
            x = y;
            y = tmp;
        }
        let master = Rc::new(Cell::new((x, y)));
        let position = Rc::downgrade(&master);
        *self.get_strong_position(x, y) = Some(master);
        Token { position }
    }

    pub fn trade(&self, token: Token) -> Option<&T> {
        token.try_get().map(|(x, y)| &self.points[y][x].0)
    }

    pub fn trade_mut(&mut self, token: Token) -> Option<&mut T> {
        token.try_get().map(move |(x, y)| &mut self.points[y][x].0)
    }

    fn get_strong_position(&mut self, x: usize, y: usize) -> &mut Option<StrongPosition> {
        &mut self.points[y][x].1
    }

    fn convert_container(points: Vec<Vec<T>>) -> Vec<Vec<(T, Option<StrongPosition>)>> {
        points
            .into_iter()
            .map(|row| row.into_iter().map(|item| (item, None)).collect())
            .collect()
    }

    fn get_internal(&self, x: usize, y: usize) -> &(T, Option<StrongPosition>) {
        if !self.is_rotated() {
            &self.points[y][x]
        } else {
            &self.points[x][y]
        }
    }

    fn get_mut_internal(&mut self, x: usize, y: usize) -> &mut (T, Option<StrongPosition>) {
        if !self.rotated {
            &mut self.points[y][x]
        } else {
            &mut self.points[x][y]
        }
    }
}

impl<T: Clone> Grid<T> {
    pub fn shift_row_left_from_point(&mut self, x: usize, y: usize) {
        for x in x..(self.width() - 1) {
            let mut clone = self.get_internal(x + 1, y).clone();
            if let Some(ref mut pos) = clone.1 {
                if !self.is_rotated() {
                    pos.set((x, y));
                } else {
                    pos.set((y, x));
                }
            }
            *self.get_mut_internal(x, y) = clone;
        }
    }

    pub fn shift_row_right_from_point(&mut self, x: usize, y: usize) {
        for x in (x + 1..self.width()).rev() {
            let mut clone = self.get_internal(x - 1, y).clone();
            if let Some(ref mut pos) = clone.1 {
                if !self.is_rotated() {
                    pos.set((x, y));
                } else {
                    pos.set((y, x));
                }
            }
            *self.get_mut_internal(x, y) = clone;
        }
    }

    // TODO handle tokens?
    pub fn add_last_column(&mut self) {
        let expect_msg = "Attempted to get last from empty grid";
        if self.rotated {
            let clone = self.points.last().expect(expect_msg).clone();
            self.points.push(clone);
        } else {
            for mut row in self.points.iter_mut() {
                let clone = row.last().expect(expect_msg).clone();
                row.push(clone);
            }
        }
    }

    fn clone_points_without_positions(&self) -> Vec<Vec<(T, Option<StrongPosition>)>> {
        let mut rows = vec![];
        for row in &self.points {
            let new_row = row.iter()
                .map(|&(ref item, _)| (item.clone(), None))
                .collect();
            rows.push(new_row);
        }
        rows
    }
}

// Manually implementing clone prevents cloned grids from updating their
// parent's tokens via Rc's that would otherwise get cloned too.
impl<T: Clone> Clone for Grid<T> {
    fn clone(&self) -> Self {
        Self {
            points: self.clone_points_without_positions(),
            rotated: self.rotated,
        }
    }
}

impl<'a> From<&'a DynamicImage> for Grid<PixelEnergyPoint> {
    fn from(image: &'a DynamicImage) -> Self {
        let (width, height) = image.dimensions();

        let mut rows = vec![];
        for y in 0..height {
            let mut row = vec![];
            for x in 0..width {
                let pixel = image.get_pixel(x, y);
                let mut pep: PixelEnergyPoint = pixel.into();
                pep.marked = true;
                pep.original_position = (x as usize, y as usize);
                row.push(pep);
            }
            rows.push(row);
        }

        Grid::new(rows)
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
