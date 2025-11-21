use wasm_bindgen::prelude::*;

mod entry;
mod ops;

pub use entry::load_image;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}
