use wasm_bindgen::prelude::*;

use crate::ops;

#[wasm_bindgen]
pub fn load_image(input: &[u8]) -> Result<u32, JsValue> {
    ops::load_image(input).map_err(|err| JsValue::from_str(&format!("{}", err)))
}

#[wasm_bindgen]
pub fn export_image(id: u32, config: &JsValue) -> Result<Vec<u8>, JsValue> {
    let image_config = serde_wasm_bindgen::from_value::<ops::config::ImageConfig>(config.clone())
        .map_err(|err| JsValue::from_str(&format!("{}", err)))?;

    ops::apply_ops(&id, &image_config).map_err(|err| JsValue::from_str(&format!("{}", err)))
}

#[wasm_bindgen]
pub fn free_image(id: u32) -> Result<(), JsValue> {
    ops::free_image(&id).map_err(|err| JsValue::from_str(&format!("{}", err)))
}
