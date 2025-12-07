use image::DynamicImage;

pub fn sharpen_op(image: &DynamicImage, sigma: f32, threshold: i32) -> DynamicImage {
    image.unsharpen(sigma, threshold)
}
