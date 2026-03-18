//! # ifcx-api
//!
//! Axum REST and WebSocket server for IFCX operations.
//!
//! ## Endpoints (from architecture diagram)
//!
//! | Method | Path | Description |
//! |--------|------|-------------|
//! | GET | /nodes | Query entities |
//! | GET | /nodes/:guid | Lazy load payload |
//! | GET | /components | Filter components |
//! | POST | /validate | Schema check |
//! | POST | /merge | Layer composition |
//! | GET | /geometry | Mesh on demand |
//! | GET | /spatial | Viewport query |
//! | GET | /cache/status | Cache hit/miss |
//! | WS | /session | Editor stream |

// TODO: Fase 2
