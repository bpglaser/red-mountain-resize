use std::collections::HashMap;
use std::ops::{Deref, Index};

#[derive(Clone,Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    fn adjacent(&self) -> (Point, Point) {
        let left = Point {
            x: self.x - 1,
            y: self.y,
        };
        let right = Point {
            x: self.x + 1,
            y: self.y,
        };
        (left, right)
    }
}

impl Deref for Point {
    type Target = (usize, usize);

    fn deref(&self) -> &Self::Target {
        unimplemented!()
    }
}

pub struct PointPath {
    points: HashMap<usize, Point>,
}

impl PointPath {
    pub fn new() -> Self {
        let points = HashMap::new();
        PointPath { points }
    }

    pub fn adjacent_points(&self) -> Vec<Point> {
        let mut adjacent_points = vec![];
        for point in self.points.values() {
            let (left, right) = point.adjacent();
            adjacent_points.push(left);
            adjacent_points.push(right);
        }
        adjacent_points
    }
}

impl Index<usize> for PointPath {
    type Output = Point;

    fn index(&self, y: usize) -> &Point {
        &self.points[&y]
    }
}
