//! # ifcx-ffi
//!
//! C ABI exports for ifcx-core. Builds to `ifcx.dll` (Windows) / `libifcx.so` (Linux).
//!
//! Consumers: Revit plugins, pyRevit, Grasshopper, Dynamo, Excel VBA, etc.

// TODO: Fase 2
//
// ```rust,ignore
// use std::ffi::{CStr, CString};
// use std::os::raw::c_char;
//
// /// Parse an IFCX JSON string and return a handle.
// #[no_mangle]
// pub extern "C" fn ifcx_parse(json: *const c_char) -> *mut IfcxHandle { ... }
//
// /// Query entries by IFC class.
// #[no_mangle]
// pub extern "C" fn ifcx_query_class(handle: *const IfcxHandle, class: *const c_char) -> *mut c_char { ... }
//
// /// Free a handle.
// #[no_mangle]
// pub extern "C" fn ifcx_free(handle: *mut IfcxHandle) { ... }
// ```
