use wasm_bindgen::prelude::*;

pub mod document;
pub mod metadata;
pub mod ops;
pub mod pipeline;

pub use document::PhoteryxDocument;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}
