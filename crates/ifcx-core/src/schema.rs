//! IFCX schema validation.
//!
//! Validates IFCX documents against the buildingSMART schema
//! and any registered custom namespace schemas.

// TODO: Fase 1 — implement schema validation
//
// Design:
//
// - Load schemas from ifcx.dev URIs (or bundled offline copies)
// - Validate document structure (header, imports, data)
// - Validate each entry's attributes against registered schemas
// - Evolution-tolerant: unknown namespaces are allowed (pass-through)
// - Report errors with path + namespace context
//
// ```rust,ignore
// pub struct SchemaValidator {
//     schemas: HashMap<String, jsonschema::JSONSchema>,
// }
//
// impl SchemaValidator {
//     pub fn validate(&self, doc: &IfcxDocument) -> Vec<ValidationError> { ... }
// }
// ```
