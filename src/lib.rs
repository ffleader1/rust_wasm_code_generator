mod utils;
mod golang_generator;
mod python_generator;
mod constant;
mod curl_handler;
mod nodejs_generator;
mod rust_generator;

use curl_handler::CurlHandler;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct WasmCurlHandlerObj(CurlHandler);

#[wasm_bindgen]
impl WasmCurlHandlerObj {
    pub fn new() -> WasmCurlHandlerObj {
        WasmCurlHandlerObj(CurlHandler::new())
    }

    pub async fn generate(&mut self, input_strings: &str, ignore_request: bool) {
        self.0.update_curl(input_strings, ignore_request).await
    }
    pub fn get_status_code(&self) -> u32 { self.0.get_status_code()}
    pub fn get_response(&self) -> String {
        self.0.get_response()
    }
    pub fn get_parsing_err(&self) -> String{
        self.0.get_parsing_err()
    }
    pub fn get_input_curl(&self) -> String {
        self.0.get_input_curl()
    }
    pub fn get_golang(&self) -> String {
        self.0.get_golang()
    }
    pub fn get_python(&self) -> String {
        self.0.get_python()
    }
    pub fn get_nodejs(&self) -> String {
        self.0.get_nodejs()
    }
    pub fn get_rust(&self) -> String {self.0.get_rust()}
}

