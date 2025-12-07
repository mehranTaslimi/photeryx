use image::DynamicImage;

pub fn invert_op(image: &DynamicImage) -> DynamicImage {
    let mut out = image.clone();
    out.invert();
    out
}
