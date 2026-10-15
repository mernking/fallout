//! EVCS Core Engine
//!
//! Pure-Rust scene model, viewport transform math, the `.evcs` ZIP container,
//! and document export routines (SVG / PNG / PDF / JSON).
//!
//! Everything here is deterministic and thread-safe so the `iced` UI and the
//! MCP CLI can mutate the same `Scene` through an `Arc<RwLock<Scene>>`.