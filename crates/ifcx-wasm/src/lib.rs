//! # ifcx-wasm
//!
//! WASM bindings for ifcx-core. Fase 3.
//!
//! Exposes the parser, query engine, and editor to JavaScript/TypeScript
//! via wasm-bindgen. Target size: ~3-6 MB.

// TODO: Fase 3
//
// ```rust,ignore
// use wasm_bindgen::prelude::*;
//
// #[wasm_bindgen]
// pub struct WasmIfcxDocument { inner: ifcx_core::IfcxDocument }
//
// #[wasm_bindgen]
// impl WasmIfcxDocument {
//     #[wasm_bindgen(constructor)]
//     pub fn parse(json: &str) -> Result<WasmIfcxDocument, JsError> { ... }
//
//     pub fn query_class(&self, class: &str) -> JsValue { ... }
//
//     pub fn to_json(&self) -> String { ... }
// }
// ```
