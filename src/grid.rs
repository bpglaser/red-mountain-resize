use math::wrap_to_bounds;

#[derive(Debug)]
pub struct Grid<T> {
    points: Vec<Vec<T>>,
    rotated: bool,
}

impl<T> Grid<T> {
    pub fn new(points: Vec<Vec<T>>) -> Self {
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

    pub fn get(&self, x: isize, y: isize) -> &T {
        let (x, y) = self.get_bounded_coords(x, y);
        if !self.rotated {
            &self.points[y][x]
        } else {
            &self.points[x][y]
        }
    }

    pub fn get_mut(&mut self, x: isize, y: isize) -> &mut T {
        let (x, y) = self.get_bounded_coords(x, y);
        if !self.rotated {
            &mut self.points[y][x]
        } else {
            &mut self.points[x][y]
        }
    }

    pub fn rotate(&mut self) {
        self.rotated = !self.rotated;
    }

    pub fn coord_iter(&self) -> PointIter<T> {
        PointIter {
            x: 0,
            y: 0,
            grid: &self,
        }
    }

    fn get_bounded_coords(&self, x: isize, y: isize) -> (usize, usize) {
        let x = wrap_to_bounds(x, 0, self.width() as isize);
        let y = wrap_to_bounds(y, 0, self.height() as isize);
        assert!(x >= 0 && y >= 0);
        (x as usize, y as usize)
    }
}

#[derive(Debug)]
pub struct PointIter<'a, T: 'a> {
    x: usize,
    y: usize,
    grid: &'a Grid<T>,
}

impl<'a, T> Iterator for PointIter<'a, T> {
    type Item = (usize, usize, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        let x = self.x;
        let y = self.y;
        let val = self.grid.get(x as isize, y as isize);

        if self.y >= self.grid.height() {
            return None;
        }

        self.x += 1;
        if self.x >= self.grid.width() {
            self.x = 0;
            self.y += 1;
        }

        Some((x, y, val))
    }
}
