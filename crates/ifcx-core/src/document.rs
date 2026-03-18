//! IFCX (IFC5) document types.
//!
//! Mirrors the IFC5 alpha spec from buildingSMART/IFC5-development.
//! An IFCX document is a flat list of data entries identified by path,
//! with typed attributes grouped by namespace.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Standard namespace constants
// ---------------------------------------------------------------------------

/// Standard buildingSMART IFC namespaces.
pub mod ns {
    pub const IFC_CLASS: &str = "bsi::ifc::class";
    pub const IFC_PROP: &str = "bsi::ifc::prop";
    pub const IFC_PRESENTATION: &str = "bsi::ifc::presentation";
    pub const IFC_MATERIAL: &str = "bsi::ifc::material";
    pub const USD_GEOM_MESH: &str = "usd::usdgeom::mesh";
}

/// IFC class codes used in `bsi::ifc::class` attributes.
pub mod ifc_class {
    pub const PROJECT: &str = "IfcProject";
    pub const SITE: &str = "IfcSite";
    pub const BUILDING: &str = "IfcBuilding";
    pub const BUILDING_STOREY: &str = "IfcBuildingStorey";
    pub const SPACE: &str = "IfcSpace";
    pub const WALL: &str = "IfcWall";
    pub const WALL_STANDARD_CASE: &str = "IfcWallStandardCase";
    pub const WINDOW: &str = "IfcWindow";
    pub const DOOR: &str = "IfcDoor";
    pub const SLAB: &str = "IfcSlab";
    pub const ROOF: &str = "IfcRoof";
    pub const COLUMN: &str = "IfcColumn";
    pub const BEAM: &str = "IfcBeam";
    pub const STAIR: &str = "IfcStair";
    pub const COVERING: &str = "IfcCovering";
    pub const CURTAIN_WALL: &str = "IfcCurtainWall";
    pub const MEMBER: &str = "IfcMember";
    pub const PLATE: &str = "IfcPlate";
    pub const RAILING: &str = "IfcRailing";
    pub const OPENING_ELEMENT: &str = "IfcOpeningElement";
    pub const BUILDING_ELEMENT_PROXY: &str = "IfcBuildingElementProxy";
    pub const DISTRIBUTION_ELEMENT: &str = "IfcDistributionElement";
    pub const FURNISHING_ELEMENT: &str = "IfcFurnishingElement";
    pub const REL_SPACE_BOUNDARY: &str = "IfcRelSpaceBoundary";
}

// ---------------------------------------------------------------------------
// Document model
// ---------------------------------------------------------------------------

/// Top-level IFCX document.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct IfcxDocument {
    pub header: IfcxHeader,
    pub imports: Vec<IfcxImport>,
    #[serde(default)]
    pub schemas: HashMap<String, serde_json::Value>,
    pub data: Vec<IfcxDataEntry>,
}

/// Document header with metadata.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IfcxHeader {
    pub id: String,
    pub ifcx_version: String,
    pub data_version: String,
    pub author: String,
    pub timestamp: String,
}

/// Schema import reference.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct IfcxImport {
    pub uri: String,
}

/// A single data entry in the IFCX flat list.
///
/// Each entry is identified by a unique `path` (typically a UUID).
/// `children` maps named slots to other entry paths.
/// `inherits` maps named slots to parent entry paths.
/// `attributes` holds namespaced data (e.g. `bsi::ifc::class`).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct IfcxDataEntry {
    pub path: String,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub children: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub inherits: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub attributes: HashMap<String, serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Constructors
// ---------------------------------------------------------------------------

impl IfcxDocument {
    /// Create a new empty IFCX document.
    pub fn new(author: &str) -> Self {
        Self {
            header: IfcxHeader {
                id: uuid::Uuid::new_v4().to_string(),
                ifcx_version: "ifcx_alpha".to_string(),
                data_version: "1.0.0".to_string(),
                author: author.to_string(),
                timestamp: String::new(),
            },
            imports: vec![IfcxImport {
                uri: "https://ifcx.dev/@standards.buildingsmart.org/ifc/core/ifc@v5a.ifcx"
                    .to_string(),
            }],
            schemas: HashMap::new(),
            data: Vec::new(),
        }
    }

    /// Find a data entry by path.
    pub fn find(&self, path: &str) -> Option<&IfcxDataEntry> {
        self.data.iter().find(|e| e.path == path)
    }

    /// Find a mutable data entry by path.
    pub fn find_mut(&mut self, path: &str) -> Option<&mut IfcxDataEntry> {
        self.data.iter_mut().find(|e| e.path == path)
    }

    /// Find all entries that have a specific IFC class.
    pub fn find_by_class(&self, class_code: &str) -> Vec<&IfcxDataEntry> {
        self.data
            .iter()
            .filter(|e| e.ifc_class() == Some(class_code))
            .collect()
    }

    /// Number of data entries.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Whether the document has no data entries.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl IfcxDataEntry {
    /// Create a new entry with the given path.
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
            children: HashMap::new(),
            inherits: HashMap::new(),
            attributes: HashMap::new(),
        }
    }

    /// Create a new entry with a random UUID path.
    pub fn with_uuid() -> Self {
        Self::new(&uuid::Uuid::new_v4().to_string())
    }

    /// Get a typed attribute value.
    pub fn get_attr<T: serde::de::DeserializeOwned>(&self, key: &str) -> Option<T> {
        self.attributes
            .get(key)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }

    /// Set a typed attribute value.
    pub fn set_attr<T: Serialize>(&mut self, key: &str, value: &T) {
        if let Ok(v) = serde_json::to_value(value) {
            self.attributes.insert(key.to_string(), v);
        }
    }

    /// Get the IFC class code (e.g. "IfcSpace") if present.
    pub fn ifc_class(&self) -> Option<&str> {
        self.attributes
            .get(ns::IFC_CLASS)
            .and_then(|v| v.get("code"))
            .and_then(|v| v.as_str())
    }

    /// Set the IFC class code.
    pub fn set_ifc_class(&mut self, code: &str) {
        self.attributes.insert(
            ns::IFC_CLASS.to_string(),
            serde_json::json!({ "code": code }),
        );
    }

    /// Get an IFC property value by name.
    pub fn ifc_prop(&self, name: &str) -> Option<&serde_json::Value> {
        self.attributes.get(ns::IFC_PROP)?.get(name)
    }

    /// Add a child entry reference.
    pub fn add_child(&mut self, name: &str, path: &str) {
        self.children.insert(name.to_string(), path.to_string());
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_document() {
        let doc = IfcxDocument::new("test");
        assert!(doc.is_empty());
        assert_eq!(doc.header.ifcx_version, "ifcx_alpha");
    }

    #[test]
    fn test_entry_class() {
        let mut entry = IfcxDataEntry::with_uuid();
        assert_eq!(entry.ifc_class(), None);

        entry.set_ifc_class("IfcWall");
        assert_eq!(entry.ifc_class(), Some("IfcWall"));
    }

    #[test]
    fn test_find_by_class() {
        let mut doc = IfcxDocument::new("test");

        let mut wall = IfcxDataEntry::with_uuid();
        wall.set_ifc_class(ifc_class::WALL);
        doc.data.push(wall);

        let mut space = IfcxDataEntry::with_uuid();
        space.set_ifc_class(ifc_class::SPACE);
        doc.data.push(space);

        assert_eq!(doc.find_by_class(ifc_class::WALL).len(), 1);
        assert_eq!(doc.find_by_class(ifc_class::SPACE).len(), 1);
        assert_eq!(doc.find_by_class(ifc_class::DOOR).len(), 0);
    }

    #[test]
    fn test_typed_attributes() {
        let mut entry = IfcxDataEntry::with_uuid();

        #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
        struct TestData {
            value: f64,
            label: String,
        }

        let data = TestData {
            value: 42.0,
            label: "test".to_string(),
        };
        entry.set_attr("my::namespace", &data);

        let restored: TestData = entry.get_attr("my::namespace").unwrap();
        assert_eq!(restored, data);
    }

    #[test]
    fn test_serialization_roundtrip() {
        let mut doc = IfcxDocument::new("roundtrip-test");
        let mut entry = IfcxDataEntry::with_uuid();
        entry.set_ifc_class(ifc_class::BUILDING);
        entry.set_attr(
            &format!("{}::Name", ns::IFC_PROP),
            &serde_json::json!("Test Building"),
        );
        doc.data.push(entry);

        let json = serde_json::to_string_pretty(&doc).unwrap();
        let restored: IfcxDocument = serde_json::from_str(&json).unwrap();

        assert_eq!(restored.len(), 1);
        assert_eq!(
            restored.data[0].ifc_class(),
            Some(ifc_class::BUILDING)
        );
    }
}
