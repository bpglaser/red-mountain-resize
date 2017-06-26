use std::iter::FromIterator;

type Point = (usize, usize);

pub struct PointStack {
    inner: Vec<Vec<Point>>,
}

impl PointStack {
    pub fn insert(&mut self, point: Point) {
        let i = self.inner.len() - 2;
        self.inner[i].push(point)
    }

    pub fn pop(&mut self) -> Option<Point> {
        match self.inner.last_mut().and_then(|row| row.pop()) {
            Some(point) => Some(point),
            None => {
                if self.inner.pop().is_some() {
                    self.pop()
                } else {
                    None
                }
            }
        }
    }
}

impl FromIterator<Point> for PointStack {
    fn from_iter<I: IntoIterator<Item = Point>>(iter: I) -> Self {
        let mut initial_points: Vec<_> = iter.into_iter().collect();
        initial_points.sort_by(|a, b| b.1.cmp(&a.1));

        let mut inner = vec![];
        let mut outer_y = None;
        let mut row = vec![];

        for point in initial_points {
            if let Some(inner_y) = outer_y {
                if point.1 != inner_y {
                    inner.push(row);
                    outer_y = Some(point.1);
                    row = vec![point];
                    continue;
                }
            }
            row.push(point);
        }

        PointStack { inner }
    }
}

impl<'a> IntoIterator for &'a PointStack {
    type Item = &'a Point;
    type IntoIter = Box<Iterator<Item=Self::Item> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.inner.iter().flat_map(|row| row.iter()))
    }
}
