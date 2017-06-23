use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl From<(usize, usize)> for Point {
    fn from(pair: (usize, usize)) -> Self {
        Point {
            x: pair.0,
            y: pair.1,
        }
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x)).reverse()
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
