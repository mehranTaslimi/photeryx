use image::DynamicImage;

pub fn rotate_op(image: &DynamicImage, degree: i16) -> DynamicImage {
    match degree {
        90 => image.rotate90(),
        180 => image.rotate180(),
        270 => image.rotate270(),
        _ => image.clone(),
    }
}
