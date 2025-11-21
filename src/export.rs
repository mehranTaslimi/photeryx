use image::DynamicImage;

use crate::ops::{
    blur_op, brightness_op, config::ImageConfig, contrast_op, grayscale_op, invert_op, rotate_op,
};

pub fn apply_ops(original: &DynamicImage, config: &ImageConfig) -> anyhow::Result<DynamicImage> {
    let mut image = original.clone();

    if let Some(rotation) = &config.rotation {
        image = rotate_op(&image, rotation.degrees);
    }

    if let Some(crop) = &config.crop
        && crop.enabled
    {
        image = crate::ops::crop_op(&image, crop);
    }

    if let Some(filter) = &config.filters {
        if filter.grayscale {
            image = grayscale_op(&image);
        }
        if filter.invert {
            image = invert_op(&image);
        }
        if let Some(blur) = filter.blur {
            image = blur_op(&image, blur);
        }
        if let Some(brightness) = filter.brightness {
            image = brightness_op(&image, brightness);
        }
        if let Some(contrast) = filter.contrast {
            image = contrast_op(&image, contrast);
        }
    }

    Ok(image)
}
