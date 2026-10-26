//! Query engine for filtering and traversing IFCX entries.
//!
//! Provides a fluent API for querying entries by class, attributes,
//! spatial bounds, and parent/child relationships.

use crate::document::{IfcxDataEntry, IfcxDocument};

/// Query builder for filtering IFCX entries.
pub struct Query<'a> {
    entries: Vec<&'a IfcxDataEntry>,
}

impl<'a> Query<'a> {
    /// Start a new query over all entries in a document.
    pub fn new(doc: &'a IfcxDocument) -> Self {
        Self {
            entries: doc.data.iter().collect(),
        }
    }

    /// Filter by IFC class code.
    pub fn by_class(mut self, class_code: &str) -> Self {
        self.entries.retain(|e| e.ifc_class() == Some(class_code));
        self
    }

    /// Filter entries that have a specific attribute namespace.
    pub fn has_attribute(mut self, namespace: &str) -> Self {
        self.entries
            .retain(|e| e.attributes.contains_key(namespace));
        self
    }

    /// Filter entries that are children of a specific parent path.
    pub fn children_of(mut self, parent_path: &str, doc: &'a IfcxDocument) -> Self {
        if let Some(parent) = doc.find(parent_path) {
            let child_paths: std::collections::HashSet<&str> = parent
                .children
                .values()
                .filter_map(|s| s.as_deref())
                .collect();
            self.entries
                .retain(|e| child_paths.contains(e.path.as_str()));
        } else {
            self.entries.clear();
        }
        self
    }

    /// Execute the query, returning matching entries.
    pub fn collect(self) -> Vec<&'a IfcxDataEntry> {
        self.entries
    }

    /// Count matching entries.
    pub fn count(self) -> usize {
        self.entries.len()
    }

    /// Get the first matching entry.
    pub fn first(self) -> Option<&'a IfcxDataEntry> {
        self.entries.into_iter().next()
    }
}

// TODO: Fase 1 — parallel query via rayon
// TODO: Fase 2 — spatial queries (BVH, viewport culling)
// TODO: Fase 2 — property aggregation (sum areas, count by type)

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::{ifc_class, IfcxDataEntry, IfcxDocument};

    fn test_doc() -> IfcxDocument {
        let mut doc = IfcxDocument::new("query-test");

        let mut building = IfcxDataEntry::new("building-1");
        building.set_ifc_class(ifc_class::BUILDING);
        building.add_child("storey-1", "storey-1");
        doc.data.push(building);

        let mut storey = IfcxDataEntry::new("storey-1");
        storey.set_ifc_class(ifc_class::BUILDING_STOREY);
        storey.add_child("space-1", "space-1");
        storey.add_child("wall-1", "wall-1");
        doc.data.push(storey);

        let mut space = IfcxDataEntry::new("space-1");
        space.set_ifc_class(ifc_class::SPACE);
        space.set_attr("isso51::room", &serde_json::json!({"function": "living"}));
        doc.data.push(space);

        let mut wall = IfcxDataEntry::new("wall-1");
        wall.set_ifc_class(ifc_class::WALL);
        doc.data.push(wall);

        doc
    }

    #[test]
    fn test_query_by_class() {
        let doc = test_doc();
        let walls = Query::new(&doc).by_class(ifc_class::WALL).collect();
        assert_eq!(walls.len(), 1);
    }

    #[test]
    fn test_query_has_attribute() {
        let doc = test_doc();
        let with_isso = Query::new(&doc).has_attribute("isso51::room").collect();
        assert_eq!(with_isso.len(), 1);
        assert_eq!(with_isso[0].ifc_class(), Some(ifc_class::SPACE));
    }

    #[test]
    fn test_query_children_of() {
        let doc = test_doc();
        let children = Query::new(&doc).children_of("storey-1", &doc).collect();
        assert_eq!(children.len(), 2); // space + wall
    }

    #[test]
    fn test_query_chained() {
        let doc = test_doc();
        let count = Query::new(&doc)
            .children_of("storey-1", &doc)
            .by_class(ifc_class::SPACE)
            .count();
        assert_eq!(count, 1);
    }
}
