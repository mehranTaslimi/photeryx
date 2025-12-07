use crate::ops::config::ResizeConfig;
use image::DynamicImage;

pub fn resize_op(image: &DynamicImage, config: &ResizeConfig) -> DynamicImage {
    image.clone()
}
