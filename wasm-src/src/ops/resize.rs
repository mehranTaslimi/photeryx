use crate::ops::config::{ResizeConfig, ResizeMode};
use image::{DynamicImage, imageops::FilterType};

pub fn resize_op(image: &DynamicImage, config: &ResizeConfig) -> DynamicImage {
    match config.mode {
        ResizeMode::Fit => image.resize(config.max_width, config.max_height, FilterType::Lanczos3),
        ResizeMode::Exact => {
            image.resize_exact(config.max_width, config.max_height, FilterType::Lanczos3)
        }
        ResizeMode::Fill => {
            image.resize_to_fill(config.max_width, config.max_height, FilterType::Lanczos3)
        }
    }
}
