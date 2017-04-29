extern crate image;
extern crate num_traits;

use self::image::{DynamicImage, GenericImage, Luma, Pixel};
use self::num_traits::cast::ToPrimitive;

use ArgConfig;
use BoxResult;

pub fn run(config: ArgConfig) -> BoxResult<()> {
    let image = image::open(&config.file_path)?;
    let carver = Carver { image };
    let energy_grid = carver.calculate_all_pixels();
    save_energy_photo(&energy_grid);
    Ok(())
}

fn save_energy_photo(energy_grid: &Vec<Vec<usize>>) {
    let height = energy_grid.len() as u32;
    let width = energy_grid[0].len() as u32;
    let mut image = DynamicImage::new_luma8(width, height).to_luma();

    let darkest_value = *energy_grid
                             .iter()
                             .flat_map(|row| row.iter())
                             .max()
                             .unwrap();

    for y in 0..height {
        for x in 0..width {
            let ratio: f64 = u8::max_value() as f64 / darkest_value as f64;
            let grid_value: usize = energy_grid[y as usize][x as usize];
            let value = grid_value as f64 * ratio;
            let pixel = Luma { data: [value as u8] };
            image.put_pixel(x, y, pixel);
        }
    }

    image.save("out.png").unwrap();
}

struct Carver<I, P>
    where I: GenericImage<Pixel = P>,
          P: Pixel
{
    image: I,
}

impl<I, P> Carver<I, P>
    where I: GenericImage<Pixel = P>,
          P: Pixel
{
    fn calculate_all_pixels(&self) -> Vec<Vec<usize>> {
        let mut grid = vec![];
        for y in 0..self.image.height() {
            let mut row = vec![];
            for x in 0..self.image.width() {
                let energy = self.calculate_pixel_energy(x, y);
                row.push(energy);
            }
            grid.push(row);
        }
        grid
    }

    fn calculate_pixel_energy(&self, x: u32, y: u32) -> usize {
        let (width, height) = self.image.dimensions();

        let left_x = x.checked_sub(1).unwrap_or(width - 1);
        let right_x = (x + 1) % width;
        let up_y = y.checked_sub(1).unwrap_or(height - 1);
        let down_y = (y + 1) % height;

        let horizontal_square_gradient = self.square_gradient(left_x, y, right_x, y);
        let vertical_square_gradient = self.square_gradient(x, up_y, x, down_y);
        horizontal_square_gradient + vertical_square_gradient
    }

    fn square_gradient(&self, x1: u32, y1: u32, x2: u32, y2: u32) -> usize {
        let pixel1 = self.image.get_pixel(x1, y1);
        let pixel1_channels = pixel1.channels();

        let pixel2 = self.image.get_pixel(x2, y2);
        let pixel2_channels = pixel2.channels();

        let mut sum = 0;
        for i in 0..pixel1_channels.len() {
            let a = pixel1_channels[i]
                .to_isize()
                .expect("Unable to convert value");
            let b = pixel2_channels[i]
                .to_isize()
                .expect("Unable to convert value");
            sum += squared_abs_difference(a, b);
        }
        sum as usize
    }
}

fn squared_abs_difference(a: isize, b: isize) -> isize {
    (a - b).abs().pow(2)
}
