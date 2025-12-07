use image::DynamicImage;

pub fn contrast_op(image: &DynamicImage, amount: f32) -> DynamicImage {
    image.adjust_contrast(amount)
}
