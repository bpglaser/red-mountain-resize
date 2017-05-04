use std::path::Path;

use image;
use image::{DynamicImage, GenericImage};

use ArgConfig;
use BoxResult;
use EnergyGrid;
use point::{Point, PointPath};

pub fn run(config: ArgConfig) -> BoxResult<()> {
    let image = image::open(&config.file_path)?;
    let mut carver = Carver::new(image);
    carver.resize_vertical(5);
    carver.save_energy_image("out.png");
    Ok(())
}

struct Carver {
    image: DynamicImage,
    energy: EnergyGrid,
}

impl Carver {
    fn new(image: DynamicImage) -> Self {
        let energy = EnergyGrid::from_image(&image);
        Carver { image, energy }
    }

    fn resize_horizontal(&mut self, distance: isize) {
        if distance < 0 {
            let distance = -distance as usize;
            for _ in 0..distance {
                self.remove_seam();
            }
            self.reduce_image_size(distance);
        } else {
            self.increase_image_size(distance as usize);
            for _ in 0..distance {
                self.add_seam();
            }
        }
    }

    fn add_seam(&mut self) {
        let path = self.energy.find_path();
        let modified = self.duplicate_path(&path);
        self.energy.add_path(&modified);
    }

    fn remove_seam(&mut self) {
        let path = self.energy.find_path();
        let modified = self.erase_path(&path);
        self.energy.remove_path(&modified);
    }

    fn duplicate_path(&mut self, path: &PointPath) -> Vec<Point> {
        let mut modified_points = vec![];
        for y in 0..self.image.height() as usize {
            let point = path[y];
            let modified_row_points = self.shift_row_right(point);
            modified_points.extend_from_slice(&modified_row_points);
        }
        modified_points
    }

    fn erase_path(&mut self, path: &PointPath) -> Vec<Point> {
        for y in 0..self.image.height() as usize {
            let point = path[y];
            self.shift_row_left(point);
        }
        path.adjacent_points()
    }

    fn shift_row_left(&mut self, point: Point) {
        let (x, y) = point.as_u32_tuple();
        for x in x..self.image.width() - 2 {
            let right_pixel = self.image.get_pixel(x + 1, y);
            self.image.put_pixel(x, y, right_pixel);
        }
    }

    fn shift_row_right(&mut self, start: Point) -> Vec<Point> {
        let mut modified = vec![start];
        let (left_x, y) = start.as_u32_tuple();
        let right_x = self.image.width() - 1;
        for x in right_x..left_x {
            modified.push(Point {
                              x: x as usize,
                              y: y as usize,
                          });
            let pixel = self.image.get_pixel(x - 1, y);
            self.image.put_pixel(x, y, pixel);
        }
        modified
    }

    fn increase_image_size(&mut self, distance: usize) {
        let (width, height) = self.image.dimensions();
        let mut new_image = DynamicImage::new_rgba8(width + distance as u32, height);
        assert!(new_image.copy_from(&self.image, 0, 0),
                "Failed to copy pixels");
        self.image = new_image;
    }

    fn reduce_image_size(&mut self, distance: usize) {
        let (width, height) = self.image.dimensions();
        self.image = self.image.crop(0, 0, width - distance as u32, height);
    }

    fn resize_vertical(&mut self, distance: isize) {
        self.rotate_clockwise();
        self.resize_horizontal(distance);
        self.rotate_counterclockwise();
    }

    fn rotate_clockwise(&mut self) {
        self.image = self.image.rotate90();
        self.energy.rotate_clockwise();
    }

    fn rotate_counterclockwise(&mut self) {
        self.image = self.image.rotate270();
        self.energy.rotate_counterclockwise();
    }

    fn save_energy_image<T: AsRef<Path>>(&self, path: T) {
        self.energy.as_image().save(path).unwrap();
    }
}
