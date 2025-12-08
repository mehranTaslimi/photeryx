use image::DynamicImage;
use imageproc::geometric_transformations::{Interpolation, rotate_about_center};

pub fn rotate_op(image: &DynamicImage, degree: i16) -> DynamicImage {
    match degree {
        0 => image.clone(),
        90 => image.rotate90(),
        180 => image.rotate180(),
        270 => image.rotate270(),
        _ => {
            let image = image.to_rgba8();

            let rotated = rotate_about_center(
                &image,
                (degree as f32).to_radians(),
                Interpolation::Bilinear,
                image::Rgba([0, 0, 0, 0]),
            );

            DynamicImage::ImageRgba8(rotated)
        }
    }
}
