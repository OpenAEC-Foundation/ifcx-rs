//! High-level edit operations.
//!
//! Each operation produces a delta layer. Operations can be undone
//! by removing the corresponding delta.

// TODO: Fase 3
//
// ```rust,ignore
// pub enum Operation {
//     AddEntity { class: String, parent: String, props: HashMap<String, Value> },
//     RemoveEntity { path: String },
//     SetAttribute { path: String, namespace: String, value: Value },
//     MoveEntity { path: String, transform: Transform },
// }
// ```
