use std::sync::atomic::AtomicU32;

use dashmap::DashMap;
use once_cell::sync::Lazy;

use crate::document::ImageDocument;

pub static ID: AtomicU32 = AtomicU32::new(0);
pub static DOCUMENTS: Lazy<DashMap<u32, ImageDocument>> = Lazy::new(|| DashMap::new());

pub enum Operation {
    Rrotate(u16),
}
