use dashmap::DashMap;
use once_cell::sync::Lazy;
use std::sync::atomic::AtomicU32;

use crate::document::ImageDocument;

mod blur;
mod brightness;
pub mod config;
mod contrast;
mod grayscale;
mod invert;
mod rotate;

pub use blur::blur_op;
pub use brightness::brightness_op;
pub use contrast::contrast_op;
pub use grayscale::grayscale_op;
pub use invert::invert_op;
pub use rotate::rotate_op;

pub static ID: AtomicU32 = AtomicU32::new(0);
pub static DOCUMENTS: Lazy<DashMap<u32, ImageDocument>> = Lazy::new(|| DashMap::new());
