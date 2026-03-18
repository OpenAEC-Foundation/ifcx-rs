//! # ifcx-editor
//!
//! Non-destructive editing of IFCX documents. Fase 3.
//!
//! ## Design
//!
//! All edits produce delta layers (small IFCX overlays) that compose
//! on top of the base document. The base document is never modified.
//! This is a native IFC5 concept.
//!
//! ```text
//! [base.ifcx] ← read-only
//!       ↓ compose
//! [edit_01.ifcx] ← delta (add wall)
//!       ↓ compose
//! [edit_02.ifcx] ← delta (move window)
//!       ↓
//! [merged view] ← runtime, lazy
//! ```
//!
//! ## Planned modules
//!
//! - `delta` — Delta layer creation and application
//! - `operations` — High-level edit commands (add/remove/modify entity)
//! - `history` — Undo/redo stack with delta layers

pub mod delta;
pub mod history;
pub mod operations;
