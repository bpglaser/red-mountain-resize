use image::{Pixel, Rgba};

use num_traits::ToPrimitive;

#[derive(Clone)]
pub struct PixelEnergyPoint {
    pub pixel: Rgba<u8>,
    pub energy: usize,
    pub path_cost: usize,
}

impl PixelEnergyPoint {
    pub fn square_gradient(&self, other: &PixelEnergyPoint) -> usize {
        let pixel1_channels = self.pixel.channels();
        let pixel2_channels = other.pixel.channels();

        let mut sum = 0;
        for i in 0..pixel1_channels.len() {
            let a = pixel1_channels[i]
                .to_isize()
                .expect("Unable to convert value");
            let b = pixel2_channels[i]
                .to_isize()
                .expect("Unable to convert value");
            sum += (a - b).abs().pow(2); // Squared abs difference
        }
        sum as usize
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
