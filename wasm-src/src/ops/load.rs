use std::sync::atomic::Ordering;

use crate::ops::{DOCUMENTS, ID};

pub fn load_image(input: &[u8]) -> anyhow::Result<u32> {
    let image = image::load_from_memory(input)?;

    let id = ID.fetch_add(1, Ordering::Relaxed);
    DOCUMENTS.insert(id, image);

    Ok(id)
}
