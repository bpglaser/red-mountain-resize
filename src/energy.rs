use image::{Pixel, Rgba};

use num_traits::ToPrimitive;

#[derive(Clone, Debug)]
pub struct PixelEnergyPoint {
    pub pixel: Rgba<u8>,
    pub energy: u32,
    pub path_cost: u32,
    pub original_position: (usize, usize),
}

impl PixelEnergyPoint {
    /// `square_gradient` serves as the basis for calculating energy.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate image;
    /// # extern crate rmr;
    /// # use image::Rgba;
    /// # use rmr::energy::PixelEnergyPoint;
    /// # fn main() {
    /// let a: PixelEnergyPoint = Rgba([255, 203, 51, 255]).into();
    /// let b: PixelEnergyPoint = Rgba([255, 205, 255, 255]).into();
    ///
    /// let result = a.square_gradient(&b);
    /// assert_eq!(41620, result);
    ///
    /// let a: PixelEnergyPoint = Rgba([255, 255, 153, 255]).into();
    /// let b: PixelEnergyPoint = Rgba([255, 153, 153, 255]).into();
    ///
    /// let result = a.square_gradient(&b);
    /// assert_eq!(10404, result);
    /// # }
    /// ```
    pub fn square_gradient(&self, other: &PixelEnergyPoint) -> u32 {
        let pixel1_channels = self.pixel.channels();
        let pixel2_channels = other.pixel.channels();

        let mut sum = 0;
        for i in 0..pixel1_channels.len() {
            // Values are cast to u32 to prevent overflow when summing
            let a = pixel1_channels[i].to_u32().unwrap();
            let b = pixel2_channels[i].to_u32().unwrap();
            if a >= b {
                sum += (a - b).pow(2);
            } else {
                sum += (b - a).pow(2);
            }
        }
        sum
    }

    pub fn average(&self, other: &PixelEnergyPoint) -> PixelEnergyPoint {
        let data = average_pixel_data(&self.pixel.0, &other.pixel.0);
        Rgba(data).into()
    }
}

impl From<Rgba<u8>> for PixelEnergyPoint {
    fn from(pixel: Rgba<u8>) -> Self {
        PixelEnergyPoint {
            pixel,
            energy: 0,
            path_cost: 0,
            original_position: (0, 0),
        }
    }
}

fn average_pixel_data(pixel1: &[u8; 4], pixel2: &[u8; 4]) -> [u8; 4] {
    [
        ((pixel1[0] as u16 + pixel2[0] as u16) / 2) as u8,
        ((pixel1[1] as u16 + pixel2[1] as u16) / 2) as u8,
        ((pixel1[2] as u16 + pixel2[2] as u16) / 2) as u8,
        ((pixel1[3] as u16 + pixel2[3] as u16) / 2) as u8,
    ]
}
