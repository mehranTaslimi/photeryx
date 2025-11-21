use image::{DynamicImage, codecs::jpeg::JpegEncoder};
use std::io::Cursor;

use crate::ops::config::{ExportConfig, ExportFormat};

pub fn export_op(image: &DynamicImage, config: &ExportConfig) -> anyhow::Result<Vec<u8>> {
    let quality = config.quality.unwrap_or(100).clamp(1, 100);

    match config.format {
        ExportFormat::Jpeg => export_jpeg(image, quality),
        ExportFormat::Webp => export_webp(image),
        ExportFormat::Png => export_png(image),
    }
}

fn export_jpeg(image: &DynamicImage, quality: u8) -> anyhow::Result<Vec<u8>> {
    let mut buf = Cursor::new(Vec::new());

    {
        let mut encoder = JpegEncoder::new_with_quality(&mut buf, quality);
        encoder.encode_image(image)?;
    }

    Ok(buf.into_inner())
}

fn export_webp(image: &DynamicImage) -> anyhow::Result<Vec<u8>> {
    let mut buf = Cursor::new(Vec::new());
    image.write_to(&mut buf, image::ImageFormat::WebP)?;
    Ok(buf.into_inner())
}

fn export_png(image: &DynamicImage) -> anyhow::Result<Vec<u8>> {
    let mut buf = Cursor::new(Vec::new());
    image.write_to(&mut buf, image::ImageFormat::Png)?;
    Ok(buf.into_inner())
}
