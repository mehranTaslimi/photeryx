use std::sync::{Arc, atomic::AtomicU32};

use dashmap::DashMap;
use image::GenericImageView;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub async fn resize(input: &[u8], x: u32, y: u32) -> Result<(), JsValue> {
    let img =
        image::load_from_memory(input).map_err(|err| JsValue::from_str(&format!("{}", err)))?;

    let d = img.dimensions();

    log(&format!(
        "Original dimensions: {:?}, resizing to: {}x{}",
        d, x, y
    ));

    Ok(())
}
