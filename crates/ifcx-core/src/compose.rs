//! Layer composition — merge multiple IFCX documents.
//!
//! IFCX supports non-destructive editing via overlays:
//! a base document (read-only) plus one or more delta layers.
//! At runtime, layers are composed into a single merged view.
//!
//! Later entries override earlier ones (last-writer-wins per attribute).

use crate::document::{IfcxDataEntry, IfcxDocument};
use std::collections::HashMap;

/// Compose multiple IFCX documents into a single flat entry list.
///
/// Later documents override earlier ones (layer composition).
/// Attributes are merged per-entry: if two layers define the same
/// attribute on the same path, the later layer wins.
pub fn compose(docs: &[&IfcxDocument]) -> Vec<IfcxDataEntry> {
    let mut merged: HashMap<String, IfcxDataEntry> = HashMap::new();

    for doc in docs {
        for entry in &doc.data {
            if let Some(existing) = merged.get_mut(&entry.path) {
                // Merge: later values override earlier
                existing.children.extend(entry.children.clone());
                existing.inherits.extend(entry.inherits.clone());
                existing.attributes.extend(entry.attributes.clone());
            } else {
                merged.insert(entry.path.clone(), entry.clone());
            }
        }
    }

    merged.into_values().collect()
}

/// Compute the delta between a base document and a modified document.
///
/// Returns only entries/attributes that differ from the base.
/// This is useful for saving edits as a minimal overlay.
pub fn diff(base: &IfcxDocument, modified: &IfcxDocument) -> Vec<IfcxDataEntry> {
    let base_map: HashMap<&str, &IfcxDataEntry> =
        base.data.iter().map(|e| (e.path.as_str(), e)).collect();

    let mut deltas = Vec::new();

    for entry in &modified.data {
        match base_map.get(entry.path.as_str()) {
            None => {
                // New entry — include entirely
                deltas.push(entry.clone());
            }
            Some(base_entry) => {
                // Existing entry — include only changed attributes
                let mut delta = IfcxDataEntry::new(&entry.path);
                let mut has_changes = false;

                for (key, value) in &entry.attributes {
                    let changed = base_entry
                        .attributes
                        .get(key)
                        .map_or(true, |base_val| base_val != value);

                    if changed {
                        delta.attributes.insert(key.clone(), value.clone());
                        has_changes = true;
                    }
                }

                for (key, value) in &entry.children {
                    if base_entry.children.get(key) != Some(value) {
                        delta.children.insert(key.clone(), value.clone());
                        has_changes = true;
                    }
                }

                if has_changes {
                    deltas.push(delta);
                }
            }
        }
    }

    deltas
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::{ifc_class, IfcxDocument};

    #[test]
    fn test_compose_two_layers() {
        let mut base = IfcxDocument::new("base");
        let mut entry = IfcxDataEntry::new("wall-1");
        entry.set_ifc_class(ifc_class::WALL);
        entry.set_attr("custom::color", &"red");
        base.data.push(entry);

        let mut overlay = IfcxDocument::new("overlay");
        let mut entry2 = IfcxDataEntry::new("wall-1");
        entry2.set_attr("custom::color", &"blue"); // override
        entry2.set_attr("custom::height", &3000.0); // new attribute
        overlay.data.push(entry2);

        let composed = compose(&[&base, &overlay]);
        assert_eq!(composed.len(), 1);

        let wall = &composed[0];
        assert_eq!(wall.ifc_class(), Some(ifc_class::WALL));

        let color: String = wall.get_attr("custom::color").unwrap();
        assert_eq!(color, "blue"); // overlay wins

        let height: f64 = wall.get_attr("custom::height").unwrap();
        assert_eq!(height, 3000.0);
    }

    #[test]
    fn test_diff_detects_changes() {
        let mut base = IfcxDocument::new("base");
        let mut entry = IfcxDataEntry::new("wall-1");
        entry.set_attr("custom::color", &"red");
        base.data.push(entry);

        let mut modified = IfcxDocument::new("modified");
        let mut entry2 = IfcxDataEntry::new("wall-1");
        entry2.set_attr("custom::color", &"blue"); // changed
        modified.data.push(entry2);

        // New entry
        let mut entry3 = IfcxDataEntry::new("wall-2");
        entry3.set_ifc_class(ifc_class::WALL);
        modified.data.push(entry3);

        let deltas = diff(&base, &modified);
        assert_eq!(deltas.len(), 2); // changed + new
    }
}
