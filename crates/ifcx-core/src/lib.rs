//! # ifcx-core
//!
//! Core library for parsing, querying, and manipulating IFCX (IFC5 JSON) documents.
//!
//! ## Design principles
//!
//! - **IFCX is the source of truth** — no shadow models, everything lives in the document
//! - **Performance first** — streaming parse, lazy loading, zero-copy binary cache
//! - **Namespace extensible** — custom namespaces via the standard IFCX mechanism
//!
//! ## Modules
//!
//! - [`document`] — IFCX document model (IfcxDocument, IfcxDataEntry, IfcxHeader)
//! - [`compose`] — Layer composition (merge multiple IFCX overlays)
//! - [`parser`] — Streaming JSON parser (future: sonic-rs SIMD)
//! - [`store`] — ECS-based in-memory store (hecs)
//! - [`query`] — Query engine for filtering and traversing entries
//! - [`schema`] — IFCX schema validation
//! - [`cache`] — Binary cache for fast re-opening (future: rkyv)
//!
//! ## Quick start
//!
//! ```rust
//! use ifcx_core::{IfcxDocument, IfcxDataEntry};
//!
//! // Parse from JSON
//! let json = r#"{
//!     "header": {
//!         "id": "test",
//!         "ifcxVersion": "ifcx_alpha",
//!         "dataVersion": "1.0.0",
//!         "author": "test",
//!         "timestamp": "2025-01-01T00:00:00Z"
//!     },
//!     "imports": [],
//!     "schemas": {},
//!     "data": []
//! }"#;
//! let doc: IfcxDocument = serde_json::from_str(json).unwrap();
//! assert!(doc.data.is_empty());
//! ```

pub mod cache;
pub mod compose;
pub mod document;
pub mod error;
pub mod parser;
pub mod query;
pub mod schema;
pub mod store;

// Re-export key types
pub use document::{IfcxDataEntry, IfcxDocument, IfcxHeader, IfcxImport};
pub use error::{IfcxError, Result};
