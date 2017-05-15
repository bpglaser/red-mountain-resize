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

    pub fn get(&self, x: usize, y: usize) -> &T {
        if !self.rotated {
            &self.points[y][x]
        } else {
            &self.points[x][y]
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

    pub fn get_parents(&self, x: usize, y: usize) -> Vec<&T> {
        let mut parents = vec![];

        if y > 0 {
            if x > 0 {
                parents.push(self.get(x - 1, y - 1));
            }

            parents.push(self.get(x, y - 1));

            if x < self.width() - 1 {
                parents.push(self.get(x + 1, y - 1));
            }
        }

        parents
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
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
        let val = self.grid.get(x, y);

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
