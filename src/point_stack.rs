use std::iter::FromIterator;
use std::slice::Iter;

type Point = (usize, usize);

pub struct PointStack {
    inner: Vec<Point>,
}

impl PointStack {
    pub fn insert(&mut self, point: Point) {
        let mut i = self.inner.len() - 1;
        loop {
            if self.inner[i].1 >= point.1 {
                self.inner.insert(i + 1, point);
                return;
            }

            if i == 0 {
                self.inner.insert(0, point);
                return;
            }

            i -= 1;
        }
    }

    pub fn pop(&mut self) -> Option<Point> {
        self.inner.pop()
    }
}

impl FromIterator<Point> for PointStack {
    fn from_iter<I: IntoIterator<Item = Point>>(iter: I) -> Self {
        let mut inner: Vec<_> = iter.into_iter().collect();
        inner.sort_by(|a, b| b.1.cmp(&a.1));
        PointStack { inner }
    }
}

impl<'a> IntoIterator for &'a PointStack {
    type Item = &'a Point;
    type IntoIter = Iter<'a, Point>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}
