use image::DynamicImage;

pub struct Grower<'a> {
    source: &'a DynamicImage,
}

impl<'a> Grower<'a> {
    pub fn new(source: &'a DynamicImage) -> Self {
        Grower { source }
    }

    pub fn create_enlarged_image(self, distance: u32) -> DynamicImage {
        unimplemented!()
    }
}
