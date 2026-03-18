# ifcx-rs — IFCX (IFC5) Toolkit

Rust-gebaseerde toolkit voor het parsen, bewerken, valideren en visualiseren van IFCX (IFC5 JSON) bestanden.

---

## Doel

Universele IFCX engine die herbruikbaar is vanuit:
- Rust libraries (andere crates)
- DLL/SO (C ABI, voor Revit/pyRevit/Grasshopper)
- Python (PyO3, voor scripts en IfcOpenShell integratie)
- WASM (browser, voor web viewers en editors)
- HTTP API (Axum, voor server-side processing)
- Tauri desktop app

---

## Architectuur

Zie `docs/ifcx-architecture-v2.html` voor het volledige diagram.

### Crates

| Crate | Doel | Fase |
|-------|------|------|
| `ifcx-core` | Parser, document model, ECS store, query engine, schema validatie, binary cache | 1 |
| `ifcx-geometry` | Geometry kernel (OpenCascade), triangulatie, LOD, BVH spatial index | 2 |
| `ifcx-editor` | Non-destructive delta layers, edit operations, undo/redo | 3 |
| `ifcx-api` | Axum REST/WebSocket server | 2 |
| `ifcx-ffi` | C ABI exports (cbindgen) | 2 |
| `ifcx-wasm` | WASM bindings (wasm-bindgen) | 3 |
| `ifcx-python` | Python bindings (PyO3) | 3 |

### Kernprincipes

- **IFCX is de enige source of truth** — geen interne modellen, alles is IFCX
- **Performance first** — 500MB+ bestanden, streaming parse, lazy loading, zero-copy cache
- **Non-destructive** — base layer (read-only) + delta layers (edits), IFC5 native concept
- **Namespace extensible** — custom namespaces (isso51::, constructief::, etc.) via plugins

---

## Belangrijke Bestanden

| Bestand | Doel |
|---------|------|
| `crates/ifcx-core/src/lib.rs` | Public API |
| `crates/ifcx-core/src/document.rs` | IFCX document model (IfcxDocument, IfcxDataEntry) |
| `crates/ifcx-core/src/parser.rs` | Streaming JSON parser (sonic-rs) |
| `crates/ifcx-core/src/store.rs` | ECS store (hecs) |
| `crates/ifcx-core/src/query.rs` | Query engine (filter, traverse) |
| `crates/ifcx-core/src/compose.rs` | Layer compositie (merge overlays) |
| `crates/ifcx-core/src/schema.rs` | IFCX schema validatie |
| `crates/ifcx-core/src/cache.rs` | rkyv binary cache |
| `crates/ifcx-geometry/src/kernel.rs` | OpenCascade BREP bridge |
| `crates/ifcx-editor/src/delta.rs` | Non-destructive delta layers |

---

## Conventies

- `cargo test` moet altijd slagen
- Eenheden: mm voor afmetingen, conform IFC
- Doc comments verwijzen naar IFCX spec (ifcx.dev)
- Geen `unsafe` in ifcx-core (behalve FFI boundaries)
- Performance benchmarks voor parser (criterion)

---

## Consumers (andere projecten die ifcx-rs gebruiken)

- **warmteverliesberekening** — ISSO 51 berekeningen, isso51:: namespace
- **OpenAEC-BIM-validator** — IFC validatie, IDS checking
- **Constructief tool** — belastingberekeningen (toekomstig)

---

## Geometry Kernel

OpenCascade (OCCT) via `opencascade-rs` crate:
- BREP operaties (boolean, fillet, chamfer)
- IFC geometry interpretatie (SweptSolid, CSG, Clipping)
- Triangulatie voor 3D rendering
- Dezelfde kernel als IfcOpenShell — geometrisch compatibel

---

## Referenties

- IFCX spec: https://ifcx.dev
- IFC5 development: https://github.com/buildingSMART/IFC5-development
- opencascade-rs: https://github.com/bschwind/opencascade-rs
- ifc-lite (MIT referentie): https://github.com/nicholasdclarke/ifc-lite
