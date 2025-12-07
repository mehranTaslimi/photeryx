use image::{DynamicImage, GenericImageView};

use crate::ops::config::CropConfig;

pub fn crop_op(image: &DynamicImage, config: &CropConfig) -> DynamicImage {
    let (img_width, img_height) = image.dimensions();

    let x = config.x.min(img_width.saturating_sub(1));
    let y = config.y.min(img_height.saturating_sub(1));

    let max_width = img_width - x;
    let max_height = img_height - y;

    let width = config.width.min(max_width);
    let height = config.height.min(max_height);

    image.crop_imm(x, y, width, height)
}
