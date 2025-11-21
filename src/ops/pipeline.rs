use crate::ops::{
    DOCUMENTS, blur_op, brightness_op, config::ImageConfig, contrast_op, export_op, grayscale_op,
    invert_op, rotate_op, sharpen_op,
};

pub fn apply_ops(id: &u32, config: &ImageConfig) -> anyhow::Result<Vec<u8>> {
    let original = DOCUMENTS.get(id).ok_or(anyhow::anyhow!("not found"))?;
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
        if let Some(sharpen) = &filter.sharpen {
            image = sharpen_op(&image, sharpen.radius, sharpen.threshold);
        }
    }

    export_op(&image, &config.export)
}
