use image::DynamicImage;

pub fn blur_op(image: &DynamicImage, sigma: f32) -> DynamicImage {
    image.blur(sigma)
}
