use wasm_bindgen::prelude::*;

pub mod document;
pub mod metadata;
pub mod ops;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
