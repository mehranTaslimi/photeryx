use std::{io::Cursor, sync::atomic::Ordering};

use image::{DynamicImage, GenericImageView, ImageFormat};
use wasm_bindgen::prelude::*;

use crate::{
    metadata::Metadata,
    ops::{DOCUMENTS, ID, Operation},
    pipeline,
};

pub struct ImageDocument {
    pub image: DynamicImage,
    pub metadata: Metadata,
    pub ops: Vec<Operation>,
}

#[wasm_bindgen]
pub struct PhoteryxDocument {
    id: u32,
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

        document.ops.push(Operation::Rotate(degrees));

        Ok(())
    }

    pub fn render(&self) -> Result<Vec<u8>, JsValue> {
        let document = DOCUMENTS
            .get(&self.id)
            .ok_or(JsValue::from_str("not found"))?;

        let img = pipeline::apply_ops(&document.image, &document.ops)
            .map_err(|err| JsValue::from_str(&format!("{}", err)))?;

        let mut buf = Cursor::new(Vec::new());

        img.write_to(&mut buf, ImageFormat::Png)
            .map_err(|err| JsValue::from_str(&format!("{}", err)))?;

        Ok(buf.into_inner())
    }

    pub fn free(self) {
        DOCUMENTS.remove(&self.id);
    }
}
