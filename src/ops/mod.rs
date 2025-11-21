use dashmap::DashMap;
use once_cell::sync::Lazy;
use std::sync::atomic::AtomicU32;

use crate::document::ImageDocument;

mod blur;
mod brightness;
mod contrast;
mod crop;
mod grayscale;
mod invert;
mod rotate;

pub mod config;

pub use blur::blur_op;
pub use brightness::brightness_op;
pub use contrast::contrast_op;
pub use crop::crop_op;
pub use grayscale::grayscale_op;
pub use invert::invert_op;
pub use rotate::rotate_op;

pub static ID: AtomicU32 = AtomicU32::new(0);
pub static DOCUMENTS: Lazy<DashMap<u32, ImageDocument>> = Lazy::new(|| DashMap::new());
