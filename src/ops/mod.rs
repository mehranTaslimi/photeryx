use dashmap::DashMap;
use image::DynamicImage;
use once_cell::sync::Lazy;
use std::sync::atomic::AtomicU32;

use crate::document::ImageDocument;

pub mod config;
pub mod rotate;

pub static ID: AtomicU32 = AtomicU32::new(0);
pub static DOCUMENTS: Lazy<DashMap<u32, ImageDocument>> = Lazy::new(|| DashMap::new());

#[derive(Debug)]
pub struct Operation {
    rotate: Option<u16>,
    crop: Option<(u64, u64)>,
}

impl Operation {
    pub fn apply(&self, image: DynamicImage) {
        // match self {
        // Operation::Rotate(degree) => {
        //     log(&format!("Applying rotate operation: {} degrees", degree));
        //     rotate::rotate_op(&image, degree)
        // }
        // }
    }
}
