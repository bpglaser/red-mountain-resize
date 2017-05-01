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
            let distance = -distance as usize;
            for _ in 0..distance {
                self.remove_seam();
            }
            self.reduce_image_size(distance);
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
        path.adjacent_points()
    }

    fn shift_row(&mut self, point: Point) {
        let (x, y) = point.as_u32_tuple();
        for x in x..self.image.width() - 2 {
            let right_pixel = self.image.get_pixel(x + 1, y);
            self.image.put_pixel(x, y, right_pixel);
        }
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
