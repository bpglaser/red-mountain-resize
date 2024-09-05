use image::{DynamicImage, GenericImage, Rgba};

pub fn create_debug_image(image: &mut DynamicImage, points: &[(usize, usize)]) -> DynamicImage {
    let red_pixel = Rgba([255, 0, 0, 255]);
    let mut image = image.clone();
    for &(x, y) in points {
        image.put_pixel(x as u32, y as u32, red_pixel);
    }
    image
}
