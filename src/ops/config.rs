use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageConfig {
    pub rotation: Option<RotationConfig>,
    pub crop: Option<CropConfig>,
    pub resize: Option<ResizeConfig>,

    #[serde(default)]
    pub filters: Option<FilterConfig>,

    pub export: ExportConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotationConfig {
    pub degrees: i16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CropConfig {
    #[serde(default)]
    pub enabled: bool,
    // #[serde(default)]
    // pub mode: CropMode,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    // pub aspect_ratio: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CropMode {
    Rect,
    CenterAspect,
    Absolute,
}

impl Default for CropMode {
    fn default() -> Self {
        CropMode::Rect
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResizeConfig {
    pub max_width: Option<u32>,
    // pub max_height: Option<u32>,

    // #[serde(default)]
    // pub mode: ResizeMode,

    // #[serde(default)]
    // pub filter: ResizeFilter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ResizeMode {
    Fit,
    Fill,
    Exact,
}

impl Default for ResizeMode {
    fn default() -> Self {
        ResizeMode::Fit
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ResizeFilter {
    Nearest,
    Triangle,
    CatmullRom,
    Gaussian,
    Lanczos3,
}

impl Default for ResizeFilter {
    fn default() -> Self {
        ResizeFilter::Lanczos3
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterConfig {
    #[serde(default)]
    pub grayscale: bool,

    #[serde(default)]
    pub invert: bool,

    pub sharpen: Option<SharpenConfig>,
    pub brightness: Option<i32>,
    pub contrast: Option<f32>,
    pub blur: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharpenConfig {
    pub radius: f32,
    pub threshold: i32,
}

impl Default for FilterConfig {
    fn default() -> Self {
        Self {
            grayscale: false,
            invert: false,
            brightness: None,
            sharpen: None,
            contrast: None,
            blur: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportConfig {
    pub format: ExportFormat,
    pub quality: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportFormat {
    Jpeg,
    Png,
    Webp,
}
