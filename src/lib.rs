extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
 }

#[wasm_bindgen]
pub fn process_message(msg: &str) -> String {
    log("Printing message");
    format!("Welcome {}", &msg)
}
