//! IFCX JSON parser.
//!
//! Currently uses serde_json for parsing. Future: sonic-rs (SIMD) for
//! 3-5x speedup on large files, and streaming parse with byte-offset
//! indexing for lazy loading.

use crate::{IfcxDocument, Result};
use std::path::Path;

/// Parse an IFCX document from a JSON string.
pub fn from_str(json: &str) -> Result<IfcxDocument> {
    Ok(serde_json::from_str(json)?)
}

/// Parse an IFCX document from a JSON byte slice.
pub fn from_slice(bytes: &[u8]) -> Result<IfcxDocument> {
    Ok(serde_json::from_slice(bytes)?)
}

/// Parse an IFCX document from a file path.
pub fn from_file(path: &Path) -> Result<IfcxDocument> {
    let bytes = std::fs::read(path)?;
    from_slice(&bytes)
}

/// Serialize an IFCX document to a JSON string (pretty-printed).
pub fn to_string(doc: &IfcxDocument) -> Result<String> {
    Ok(serde_json::to_string_pretty(doc)?)
}

/// Serialize an IFCX document to a file.
pub fn to_file(doc: &IfcxDocument, path: &Path) -> Result<()> {
    let json = to_string(doc)?;
    std::fs::write(path, json)?;
    Ok(())
}

// TODO: Fase 1 — streaming parser
//
// pub struct StreamingParser { ... }
//
// - Uses memmap2 to memory-map the file (no heap allocation)
// - First pass: build byte-offset index (path → offset in file)
// - Lazy load: only parse entry payload when accessed
// - sonic-rs for SIMD-accelerated JSON parsing

#[cfg(test)]
mod tests {
    use super::*;
    use crate::IfcxDocument;

    #[test]
    fn test_parse_minimal() {
        let json = r#"{
            "header": {
                "id": "test",
                "ifcxVersion": "ifcx_alpha",
                "dataVersion": "1.0.0",
                "author": "test",
                "timestamp": "2025-01-01T00:00:00Z"
            },
            "imports": [],
            "schemas": {},
            "data": []
        }"#;

        let doc = from_str(json).unwrap();
        assert!(doc.is_empty());
        assert_eq!(doc.header.author, "test");
    }

    #[test]
    fn test_roundtrip_file() {
        let doc = IfcxDocument::new("file-test");
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.ifcx");

        to_file(&doc, &path).unwrap();
        let restored = from_file(&path).unwrap();

        assert_eq!(restored.header.ifcx_version, doc.header.ifcx_version);
    }
}

#[cfg(test)]
mod lenient_tests {
    use super::*;

    #[test]
    fn a_file_with_a_minimal_header_and_no_imports_reads() {
        // only a version in the header, no imports, and a field this crate does not know
        let doc = from_str(r#"{"header":{"ifcxVersion":"ifcx_alpha"},"data":[{"path":"a","children":{"b":null},"attributes":{"bsi::ifc::class":{"code":"IfcWall"}},"extra":1}]}"#).unwrap();
        assert_eq!(doc.header.ifcx_version, "ifcx_alpha");
        assert!(doc.header.author.is_empty() && doc.imports.is_empty());
        let entry = &doc.data[0];
        assert_eq!(entry.ifc_class(), Some("IfcWall"));
        // an explicit null keeps its place: it removes child b of a weaker layer
        assert_eq!(entry.children.get("b"), Some(&None));
        assert!(entry.additional.contains_key("extra"));
        // without any header at all
        assert!(from_str(r#"{"data":[]}"#).unwrap().data.is_empty());
    }
}
