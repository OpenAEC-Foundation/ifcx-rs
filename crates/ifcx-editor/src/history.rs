//! Undo/redo history using delta layers.
//!
//! Each operation in the history is a delta IFCX document.
//! Undo = remove the top delta. Redo = re-apply it.

// TODO: Fase 3
