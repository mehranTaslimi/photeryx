use image::ImageFormat;

pub struct Metadata {
    pub width: u32,
    pub height: u32,
    pub format: Option<ImageFormat>,
    pub color_type: image::ColorType,
}
