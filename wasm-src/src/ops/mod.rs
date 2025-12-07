use dashmap::DashMap;
use image::DynamicImage;
use once_cell::sync::Lazy;
use std::sync::atomic::AtomicU32;

mod blur;
mod brightness;
mod contrast;
mod crop;
mod export;
mod grayscale;
mod invert;
mod load;
mod pipeline;
mod resize;
mod rotate;
mod sharpen;

pub mod config;

pub use load::load_image;
pub use pipeline::apply_ops;

pub use blur::blur_op;
pub use brightness::brightness_op;
pub use contrast::contrast_op;
pub use crop::crop_op;
pub use export::export_op;
pub use grayscale::grayscale_op;
pub use invert::invert_op;
pub use resize::resize_op;
pub use rotate::rotate_op;
pub use sharpen::sharpen_op;

pub static ID: AtomicU32 = AtomicU32::new(0);
pub static DOCUMENTS: Lazy<DashMap<u32, DynamicImage>> = Lazy::new(|| DashMap::new());
