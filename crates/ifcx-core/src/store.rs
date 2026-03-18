//! ECS-based in-memory store for IFCX data.
//!
//! Uses `hecs` for a struct-of-arrays Entity Component System.
//! Each IFCX data entry becomes an entity with components for
//! class, properties, geometry, etc.
//!
//! This provides cache-friendly iteration and fast queries.

// TODO: Fase 1 — implement ECS store
//
// Design:
//
// ```rust,ignore
// use hecs::World;
//
// /// Core components
// struct IfcClass(String);           // e.g. "IfcWall"
// struct IfcPath(String);            // UUID path
// struct IfcProps(HashMap<String, serde_json::Value>);
// struct IfcChildren(Vec<hecs::Entity>);
// struct IfcParent(hecs::Entity);
// struct SpatialBounds(BoundingBox);
//
// /// The IFCX store wraps a hecs World
// pub struct IfcxStore {
//     world: World,
//     path_index: HashMap<String, hecs::Entity>,
//     class_index: HashMap<String, Vec<hecs::Entity>>,
// }
//
// impl IfcxStore {
//     pub fn from_document(doc: &IfcxDocument) -> Self { ... }
//     pub fn query_class(&self, class: &str) -> Vec<hecs::Entity> { ... }
//     pub fn get_path(&self, path: &str) -> Option<hecs::Entity> { ... }
// }
// ```
