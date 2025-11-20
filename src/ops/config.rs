use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageConfig {
    pub rotation: Option<RotationConfig>,
    pub crop: Option<CropConfig>,
    pub resize: Option<ResizeConfig>,

    #[serde(default)]
    pub filters: Option<FilterConfig>,

    pub background: Option<BackgroundConfig>,
    pub sharpen: Option<SharpenConfig>,
    pub export: Option<ExportConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotationConfig {
    pub degrees: i16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CropConfig {
    #[serde(default)]
    pub enabled: bool,

    #[serde(default)]
    pub mode: CropMode,

    pub x: Option<u32>,
    pub y: Option<u32>,
    pub width: Option<u32>,
    pub height: Option<u32>,

    pub aspect_ratio: Option<f32>,
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
    pub max_height: Option<u32>,

    #[serde(default)]
    pub mode: ResizeMode,

    #[serde(default)]
    pub filter: ResizeFilter,
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

    pub brightness: Option<i32>,
    pub contrast: Option<f32>,
    pub blur: Option<f32>,
}

impl Default for FilterConfig {
    fn default() -> Self {
        Self {
            grayscale: false,
            invert: false,
            brightness: None,
            contrast: None,
            blur: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundConfig {
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharpenConfig {
    pub amount: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportConfig {
    pub format: ExportFormat,
    pub quality: Option<u8>,

    #[serde(default)]
    pub strip_metadata: bool,

    #[serde(default)]
    pub force_rgb: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportFormat {
    Jpeg,
    Png,
    Webp,
}
