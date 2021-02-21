mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn verify() {
    log("WASM loaded");
}

#[wasm_bindgen]
pub struct AppClient {

}

#[wasm_bindgen]
impl AppClient {
    
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        log("AppClient Constructor called");
        Self {

        }
    }

    pub fn update(&mut self, _time: f32, _height: f32, _width: f32) -> Result<(), JsValue> {
        log("AppClient.update called");
        Ok(())
    }

    pub fn render(&self) {
        log("AppClient.render called");
    }

}