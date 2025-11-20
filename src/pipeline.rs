use image::DynamicImage;

use crate::ops::Operation;

pub fn apply_ops(original: &DynamicImage, ops: &[Operation]) -> anyhow::Result<DynamicImage> {
    let mut image = original.clone();

    for op in ops {
        image = op.apply(image);
    }

    Ok(image)
}
