use std::cell::Cell;
use std::rc::{Rc, Weak};

use image::{DynamicImage, GenericImageView};

use crate::energy::PixelEnergyPoint;

type StrongPosition = Rc<Cell<(usize, usize)>>;
type WeakPosition = Weak<Cell<(usize, usize)>>;

#[derive(Clone)]
pub struct Token {
    position: WeakPosition,
}

impl Token {
    fn try_get(&self) -> Option<(usize, usize)> {
        self.position.upgrade().map(|p| p.get())
    }
}

#[derive(Clone)]
struct Item<T> {
    val: T,
    pos: Option<StrongPosition>,
}

impl<T> Item<T> {
    fn update_pos(&mut self, x: usize, y: usize) {
        if let Some(ref mut pos) = self.pos {
            pos.set((x, y));
        }
    }
}

impl<T: Clone> Item<T> {
    fn clone_unindexed(&self) -> Item<T> {
        Item {
            val: self.val.clone(),
            pos: None,
        }
    }
}

pub struct Grid<T> {
    points: Vec<Vec<Item<T>>>,
    rotated: bool,
}

impl<T> Grid<T> {
    pub fn new(points: Vec<Vec<T>>) -> Self {
        let points = Grid::convert_container(points);
        let rotated = false;
        Self { points, rotated }
    }

    pub fn height(&self) -> usize {
        self.points.len()
    }

    pub fn width(&self) -> usize {
        self.points[0].len()
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.points[y][x].val
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

    pub fn iter_parents(&self, x: usize, y: usize) -> impl Iterator<Item = &T> {
        self.iter_parents_with_coords(x, y).map(|(_, item)| item)
    }

    pub fn get_parents_indexed(&self, x: usize, y: usize) -> Vec<((usize, usize), &T)> {
        self.iter_parents_with_coords(x, y).collect()
    }

    pub fn iter_parents_with_coords(&self, x: usize, y: usize) -> ParentIter<T> {
        ParentIter::new(self, x, y)
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
        &mut self.points[y][x].val
    }

    pub fn rotate(&mut self) {
        self.rotated = !self.rotated;

        let mut rows = vec![];
        'outer: loop {
            let mut row = vec![];
            for cur in &mut self.points {
                match cur.pop() {
                    Some(val) => row.push(val),
                    None => break 'outer,
                }
            }
            rows.push(row);
        }
        rows.reverse();

        self.points = rows;
    }

    pub fn is_rotated(&self) -> bool {
        self.rotated
    }

    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a T> + 'a> {
        Box::new(
            self.points
                .iter()
                .flat_map(|row| row.iter())
                .map(|item| &item.val),
        )
    }

    pub fn iter_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &'a mut T> + 'a> {
        Box::new(
            self.points
                .iter_mut()
                .flat_map(|row| row.iter_mut())
                .map(|item| &mut item.val),
        )
    }

    pub fn coord_iter<'a>(&'a self) -> Box<dyn Iterator<Item = (usize, usize, &'a T)> + 'a> {
        let rotated = self.is_rotated();
        Box::new(self.points.iter().enumerate().flat_map(move |(y, row)| {
            row.iter().enumerate().map(move |(x, item)| {
                if !rotated {
                    (x, y, &item.val)
                } else {
                    (y, x, &item.val)
                }
            })
        }))
    }

    pub fn coord_iter_mut<'a>(
        &'a mut self,
    ) -> Box<dyn Iterator<Item = (usize, usize, &'a mut T)> + 'a> {
        let rotated = self.is_rotated();
        Box::new(
            self.points
                .iter_mut()
                .enumerate()
                .flat_map(move |(y, row)| {
                    row.iter_mut().enumerate().map(move |(x, item)| {
                        if !rotated {
                            (x, y, &mut item.val)
                        } else {
                            (y, x, &mut item.val)
                        }
                    })
                }),
        )
    }

    pub fn remove_last_column(&mut self) {
        for row in &mut self.points {
            row.pop()
                .expect("Attempted to remove column from empty grid");
        }
    }

    pub fn make_token(&mut self, x: usize, y: usize) -> Token {
        let master = Rc::new(Cell::new(self.rotate_point((x, y))));
        let position = Rc::downgrade(&master);
        self.points[y][x].pos = Some(master);
        Token { position }
    }

    pub fn make_adjacent_tokens(&mut self, x: usize, y: usize) -> [Token; 4] {
        let x_left = if x == 0 { self.width() - 1 } else { x - 1 };
        let left = self.make_token(x_left, y);

        let x_right = if x == self.width() - 1 { 0 } else { x + 1 };
        let right = self.make_token(x_right, y);

        let y_up = if y == 0 { self.height() - 1 } else { y - 1 };
        let up = self.make_token(x, y_up);

        let y_down = if y == self.height() - 1 { 0 } else { y + 1 };
        let down = self.make_token(x, y_down);

        [left, right, up, down]
    }

    pub fn trade(&self, token: Token) -> Option<&T> {
        token.try_get().map(|(x, y)| &self.get_internal(x, y).val)
    }

    pub fn trade_mut(&mut self, token: Token) -> Option<&mut T> {
        token
            .try_get()
            .map(move |(x, y)| &mut self.get_mut_internal(x, y).val)
    }

    pub fn get_token_adjacent(&self, token: &Token) -> Option<(&T, &T, &T, &T)> {
        token
            .try_get()
            .map(|point| self.rotate_point(point))
            .map(|(x, y)| self.get_adjacent(x, y))
    }

    fn rotate_point(&self, point: (usize, usize)) -> (usize, usize) {
        if !self.is_rotated() {
            point
        } else {
            (point.1, point.0)
        }
    }

    fn convert_container(points: Vec<Vec<T>>) -> Vec<Vec<Item<T>>> {
        points
            .into_iter()
            .map(|row| row.into_iter().map(|val| Item { val, pos: None }).collect())
            .collect()
    }

    fn get_internal(&self, x: usize, y: usize) -> &Item<T> {
        if !self.is_rotated() {
            &self.points[y][x]
        } else {
            &self.points[x][y]
        }
    }

    fn get_mut_internal(&mut self, x: usize, y: usize) -> &mut Item<T> {
        if !self.is_rotated() {
            &mut self.points[y][x]
        } else {
            &mut self.points[x][y]
        }
    }
}

impl<T: Clone> Grid<T> {
    pub fn shift_row_left_from_point(&mut self, x: usize, y: usize) {
        for x in x..(self.width() - 1) {
            let mut clone = self.points[y][x + 1].clone();
            if !self.is_rotated() {
                clone.update_pos(x, y);
            } else {
                clone.update_pos(y, x);
            }
            self.points[y][x] = clone;
        }
    }

    pub fn shift_row_right_from_point(&mut self, x: usize, y: usize) {
        for x in (x + 1..self.width()).rev() {
            let mut clone = self.points[y][x - 1].clone();
            if !self.is_rotated() {
                clone.update_pos(x, y);
            } else {
                clone.update_pos(y, x);
            }
            self.points[y][x] = clone;
        }
    }

    pub fn add_last_column(&mut self) {
        for row in &mut self.points {
            let clone = row
                .last()
                .expect("Attempted to get last from empty grid")
                .clone();
            row.push(clone);
        }
    }

    fn clone_points_without_positions(&self) -> Vec<Vec<Item<T>>> {
        let mut rows = vec![];
        for row in &self.points {
            let new_row = row.iter().map(|item| item.clone_unindexed()).collect();
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
                pep.original_position = (x as usize, y as usize);
                row.push(pep);
            }
            rows.push(row);
        }

        Grid::new(rows)
    }
}

pub enum ParentIter<'grid, T: 'grid> {
    Done,
    Right {
        x: usize,
        y: usize,
        grid: &'grid Grid<T>,
    },
    Middle {
        x: usize,
        y: usize,
        grid: &'grid Grid<T>,
    },
    Left {
        x: usize,
        y: usize,
        grid: &'grid Grid<T>,
    },
}

impl<'grid, T> ParentIter<'grid, T> {
    fn new(grid: &'grid Grid<T>, mut x: usize, mut y: usize) -> Self {
        if y == 0 {
            // We can't move up. We're already done.
            return ParentIter::Done;
        }
        // Move up.
        y -= 1;
        if x == 0 {
            // We can't move left. So this cell only has two parents.
            return ParentIter::Middle { x, y, grid };
        }
        // Move left.
        x -= 1;
        ParentIter::Left { x, y, grid }
    }
}

impl<'grid, T> Iterator for ParentIter<'grid, T> {
    type Item = ((usize, usize), &'grid T);

    fn next(&mut self) -> Option<Self::Item> {
        let res;
        match self {
            ParentIter::Done => return None,
            ParentIter::Right { x, y, grid } => {
                let (x, y) = (*x, *y);
                res = Some(((x, y), grid.get(x, y)));
                *self = ParentIter::Done;
            }
            ParentIter::Middle { x, y, grid } => {
                let (x, y) = (*x, *y);
                res = Some(((x, y), grid.get(x, y)));
                if x == grid.width() - 1 {
                    // The iterator can't move right without falling off the grid.
                    *self = ParentIter::Done;
                } else {
                    *self = ParentIter::Right { x: x + 1, y, grid };
                }
            }
            ParentIter::Left { x, y, grid } => {
                let (x, y) = (*x, *y);
                res = Some(((x, y), grid.get(x, y)));
                if x == grid.width() - 1 {
                    // The iterator can't move right without falling off the grid.
                    // This shouldn't happen unless the grid somehow changed.
                    *self = ParentIter::Done;
                } else {
                    *self = ParentIter::Middle { x: x + 1, y, grid };
                }
            }
        }
        res
    }
}
