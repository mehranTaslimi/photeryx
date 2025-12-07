use image::DynamicImage;

pub fn brightness_op(image: &DynamicImage, amount: i32) -> DynamicImage {
    image.brighten(amount)
}
