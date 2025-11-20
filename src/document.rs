use std::{io::Cursor, sync::atomic::Ordering};

use image::{DynamicImage, GenericImageView, ImageFormat};
use wasm_bindgen::prelude::*;

use crate::{
    metadata::Metadata,
    ops::{DOCUMENTS, ID, config::ImageConfig},
    process,
};

pub struct ImageDocument {
    pub image: DynamicImage,
    pub metadata: Metadata,
    pub config: ImageConfig,
}

#[wasm_bindgen]
pub struct PhoteryxDocument {
    id: u32,
}

#[wasm_bindgen]
impl PhoteryxDocument {
    #[wasm_bindgen(constructor)]
    pub fn new(input: &[u8], config: &JsValue) -> Result<Self, JsValue> {
        let image =
            image::load_from_memory(input).map_err(|err| JsValue::from_str(&format!("{}", err)))?;

        let (width, height) = image.dimensions();

        let image_config = serde_wasm_bindgen::from_value::<ImageConfig>(config.clone())
            .map_err(|err| JsValue::from_str(&format!("{}", err)))?;

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
            config: image_config,
        };

        DOCUMENTS.insert(id, image);

        Ok(Self { id })
    }

    pub fn process(&self) -> Result<Vec<u8>, JsValue> {
        let document = DOCUMENTS
            .get(&self.id)
            .ok_or(JsValue::from_str("not found"))?;

        let img = process::apply_process(&document.image, &document.config)
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
