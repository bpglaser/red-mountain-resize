use std::collections::HashMap;
use std::ops::Index;

#[derive(Clone,Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn as_tuple(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn as_u32_tuple(&self) -> (u32, u32) {
        (self.x as u32, self.y as u32)
    }

    // Returns points to the left and right of itself.
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
