type Point = (usize, usize);

#[derive(Clone)]
pub struct PointStack {
    path: Vec<Option<Point>>,
    next_row: Vec<Point>,
}

impl PointStack {
    pub fn new() -> Self {
        PointStack {
            path: vec![],
            next_row: vec![],
        }
    }

    pub fn insert(&mut self, point: Point) {
        self.next_row.push(point)
    }

    pub fn pop(&mut self) -> Option<Point> {
        if self.path.is_empty() {
            return None;
        } else {
            if self.path.last().unwrap().is_some() {
                return self.path.pop().unwrap();
            }
        }

        match self.next_row.pop() {
            Some(point) => Some(point),
            None => {
                self.path.pop();
                self.pop()
            }
        }
    }

    pub fn fill_path(&mut self, mut points: Vec<Point>) {
        points.sort_by(|a, b| b.1.cmp(&a.1));

        let mut prev_y = points[0].1;
        for point in points {
            if prev_y != point.1 {
                prev_y = point.1;
                self.path.push(None);
            }
            self.path.push(Some(point));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation() {
        unimplemented!()
    }
}
