use dashmap::DashMap;
use image::DynamicImage;
use once_cell::sync::Lazy;
use std::sync::atomic::AtomicU32;

use crate::{document::ImageDocument, log};

pub mod rotate;

pub static ID: AtomicU32 = AtomicU32::new(0);
pub static DOCUMENTS: Lazy<DashMap<u32, ImageDocument>> = Lazy::new(|| DashMap::new());

#[derive(Debug)]
pub enum Operation {
    Rotate(u16),
}

impl Operation {
    pub fn apply(&self, image: DynamicImage) -> DynamicImage {
        match self {
            Operation::Rotate(degree) => {
                log(&format!("Applying rotate operation: {} degrees", degree));
                rotate::rotate_op(&image, degree)
            }
        }
    }
}
