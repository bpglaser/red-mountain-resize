use std::iter::FromIterator;

use itertools::Itertools;

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

        let inner = initial_points
            .into_iter()
            .group_by(|&(_, y)| y)
            .into_iter()
            .map(|(_, group)| group.collect())
            .collect();

        PointStack { inner }
    }
}

impl<'a> IntoIterator for &'a PointStack {
    type Item = &'a Point;
    type IntoIter = Box<Iterator<Item = Self::Item> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.inner.iter().flat_map(|row| row.iter()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation() {
        let point_stack: PointStack = [(0, 0), (0, 1), (1, 1), (0, 2), (0, 3), (1, 3), (0, 4),
                                       (1, 4)]
                .iter()
                .cloned()
                .collect();

        assert_eq!(vec![vec![(0, 4), (1, 4)],
                        vec![(0, 3), (1, 3)],
                        vec![(0, 2)],
                        vec![(0, 1), (1, 1)],
                        vec![(0, 0)]],
                   point_stack.inner);
    }
}
