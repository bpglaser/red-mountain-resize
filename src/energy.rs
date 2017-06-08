use image::{Pixel, Rgba};

use num_traits::ToPrimitive;

#[derive(Clone)]
pub struct PixelEnergyPoint {
    pub pixel: Rgba<u8>,
    pub energy: u32,
    pub path_cost: u32,
}

impl PixelEnergyPoint {
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
}

impl From<Rgba<u8>> for PixelEnergyPoint {
    fn from(pixel: Rgba<u8>) -> Self {
        PixelEnergyPoint {
            pixel,
            energy: 0,
            path_cost: 0,
        }
    }
}
