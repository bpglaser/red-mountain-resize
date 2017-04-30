use std::path::Path;

use image;
use image::{DynamicImage, GenericImage};

use ArgConfig;
use BoxResult;
use EnergyGrid;
use point::{Point, PointPath};

pub fn run(config: ArgConfig) -> BoxResult<()> {
    let image = image::open(&config.file_path)?;
    let carver = Carver::new(image);
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
            for _ in 0..-distance {
                self.remove_seam();
            }
        } else {
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
        unimplemented!()
    }

    fn erase_path(&mut self, path: &PointPath) -> Vec<Point> {
        for y in 0..self.image.height() as usize {
            let point = path[y];
            self.shift_row(point);
        }
        self.trim_last_column();
        path.adjacent_points()
    }

    fn shift_row(&mut self, point: Point) {
        for x in point.x..self.image.width() as usize - 2 {}
        unimplemented!()
    }

    fn trim_last_column(&mut self) {
        unimplemented!()
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
