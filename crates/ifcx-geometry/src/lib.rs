//! # ifcx-geometry
//!
//! Geometry kernel for IFCX documents. Fase 2.
//!
//! ## Planned modules
//!
//! - `kernel` — OpenCascade BREP bridge (SweptSolid, CSG, Clipping, Booleans)
//! - `triangulate` — BREP → triangle mesh for rendering
//! - `lod` — Level-of-detail generation (LOD 0: bbox, LOD 1: simplified, LOD 2: full)
//! - `spatial` — BVH spatial index for viewport culling
//! - `export` — glTF / Draco export

pub mod kernel;
pub mod triangulate;
pub mod lod;
pub mod spatial;
