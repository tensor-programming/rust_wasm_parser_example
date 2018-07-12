#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

mod parser;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(input: &str) -> String {
    let result = parser::parse(input.to_string());
    result
}
