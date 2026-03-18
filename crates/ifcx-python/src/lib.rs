//! # ifcx-python
//!
//! Python bindings for ifcx-core via PyO3. Fase 3.
//!
//! ```python
//! import ifcx
//!
//! doc = ifcx.parse("model.ifcx")
//! walls = doc.query_class("IfcWall")
//! print(f"Found {len(walls)} walls")
//! ```

// TODO: Fase 3
//
// ```rust,ignore
// use pyo3::prelude::*;
//
// #[pyclass]
// struct PyIfcxDocument { inner: ifcx_core::IfcxDocument }
//
// #[pymethods]
// impl PyIfcxDocument {
//     #[staticmethod]
//     fn parse(json: &str) -> PyResult<Self> { ... }
//
//     fn query_class(&self, class: &str) -> PyResult<Vec<PyIfcxEntry>> { ... }
// }
//
// #[pymodule]
// fn ifcx(m: &Bound<'_, PyModule>) -> PyResult<()> {
//     m.add_class::<PyIfcxDocument>()?;
//     Ok(())
// }
// ```
