//! EVCS Core Engine
//!
//! Pure-Rust scene model, viewport transform math, the `.evcs` ZIP container,
//! and document export routines (SVG / PNG / PDF / JSON).
//!
//! Everything here is deterministic and thread-safe so the `iced` UI and the
//! MCP CLI can mutate the same `Scene` through an `Arc<RwLock<Scene>>`.

mod archive;
mod export;
mod scene;

pub use archive::{open_archive, save_archive};
pub use export::{export_json, export_png, export_pdf, export_svg};
pub use scene::{Camera, Color, Document, Layer, Point, Scene, Stroke};