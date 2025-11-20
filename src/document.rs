use std::sync::atomic::Ordering;

use image::{DynamicImage, GenericImageView};
use wasm_bindgen::prelude::*;

use crate::{
    metadata::Metadata,
    ops::{DOCUMENTS, ID, Operation},
};

#[wasm_bindgen]
pub struct PhoteryxDocument {
    id: u32,
}

pub struct ImageDocument {
    image: DynamicImage,
    metadata: Metadata,
    ops: Vec<Operation>,
}

#[wasm_bindgen]
impl PhoteryxDocument {
    #[wasm_bindgen(constructor)]
    pub fn new(input: &[u8]) -> Result<Self, JsValue> {
        let image =
            image::load_from_memory(input).map_err(|err| JsValue::from_str(&format!("{}", err)))?;

        let (width, height) = image.dimensions();

        let metadata = Metadata {
            width,
            height,
            format: None,
            color_type: image.color(),
        };

        let id = ID.fetch_add(1, Ordering::Relaxed);
        let image = ImageDocument {
            image,
            metadata,
            ops: vec![],
        };

        DOCUMENTS.insert(id, image);

        Ok(Self { id })
    }

    pub fn rotate(&self, degrees: u16) -> Result<(), JsValue> {
        let mut document = DOCUMENTS
            .get_mut(&self.id)
            .ok_or(JsValue::from_str("not found"))?;

        document.ops.push(Operation::Rrotate(degrees));

        Ok(())
    }

    pub fn render() {}

    pub fn free(self) {
        DOCUMENTS.remove(&self.id);
    }
}
