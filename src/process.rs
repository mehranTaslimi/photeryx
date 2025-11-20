use image::DynamicImage;

use crate::ops::config::ImageConfig;

pub fn apply_process(
    original: &DynamicImage,
    config: &ImageConfig,
) -> anyhow::Result<DynamicImage> {
    let mut image = original.clone();

    // for op in ops {
    //     image = op.apply(image);
    // }

    Ok(image)
}
