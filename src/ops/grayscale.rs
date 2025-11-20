use image::DynamicImage;

pub fn grayscale_op(image: &DynamicImage) -> DynamicImage {
    image.grayscale()
}
