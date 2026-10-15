# Phase 0 Design: System Foundation & Core Rendering Architecture

## Overview

Establish the platform infrastructure, native windowing, spatial acceleration structures, and file system primitives for the Engineering Vector Canvas System (EVCS). This phase creates the foundation upon which all subsequent phases build.

## Architecture

### 1. Application Shell (`app.rs`)

The application uses `iced` 0.14 with the `widget::canvas` module backed by wgpu for hardware-accelerated 2D rendering. The main window contains:

- **Canvas area**: Infinite scrollable/pannable vector drawing surface
- **Status bar**: Coordinate readout, mode indicators
- **Color scheme**: `#F8F9FA` background, `#1A1A1A` strokes, `#0066FF` accent

### 2. Scene Model (`engine/scene.rs`)

Core data types for the vector canvas:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub pressure: Option<f64>,   // 0.0–1.0 from stylus
    pub tilt_x: Option<f64>,     // -1.0 to 1.0
    pub tilt_y: Option<f64>,     // -1.0 to 1.0
    pub timestamp: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color {
    pub r: f64,  // sRGB 0.0–1.0
    pub g: f64,
    pub b: f64,
    pub a: f64,  // alpha, 1.0 = opaque
}

#[derive(Debug, Clone)]
pub struct Stroke {
    pub id: u64,
    pub layer_id: u64,
    pub points: Vec<Point>,
    pub color: Color,
    pub width: f64,       // base width in pixels
    pub visible: bool,
}

#[derive(Debug, Clone)]
pub struct Layer {
    pub id: u64,
    pub name: String,
    pub visible: bool,
    pub strokes: Vec<u64>,  // references into Scene::strokes
}

#[derive(Debug, Clone)]
pub struct Camera {
    pub center: Point,
    pub scale: f64,    // zoom level (1.0 = 1 unit = 1px at default resolution)
    pub rotation: f64, // radians
}

#[derive(Debug, Clone)]
pub struct Scene {
    pub layers: Vec<Layer>,
    pub camera: Camera,
    pub stroke_indices: rstar::RTree<rstar::AABB<[f64; 2]>>,
    pub next_stroke_id: u64,
}
```

### 3. Viewport Transform

The core math for converting world coordinates to screen coordinates:

```rust
#[derive(Debug, Clone, Copy)]
pub struct ViewportTransform {
    pub scale: f64,
    pub offset: Point,
    pub rotation: f64, // radians
}

impl ViewportTransform {
    pub fn world_to_screen(&self, world: Point) -> Point {
        // Apply rotation, scale, then translate to screen center
        let rotated = Point {
            x: world.x * self.rotation.cos() - world.y * self.rotation.sin(),
            y: world.x * self.rotation.sin() + world.y * self.rotation.cos(),
        };
        Point {
            x: (rotated.x * self.scale) + self.offset.x,
            y: (rotated.y * self.scale) + self.offset.y,
        }
    }
    
    pub fn screen_to_world(&self, screen: Point) -> Point {
        // Reverse transformation
        let translated = Point {
            x: (screen.x - self.offset.x) / self.scale,
            y: (screen.y - self.offset.y) / self.scale,
        };
        Point {
            x: translated.x * self.rotation.cos() + translated.y * self.rotation.sin(),
            y: -translated.x * self.rotation.sin() + translated.y * self.rotation.cos(),
        }
    }
}
```

### 4. Archive Format (`engine/archive.rs`)

The `.evcs` ZIP container structure:

```
my_project.evcs (ZIP Archive)
├── manifest.json       # Version, metadata, engine compatibility
├── scene.json          # Layer hierarchy, active constraints, camera vectors
├── strokes.bin         # Binary packed raw/smoothed point data
└── assets/             # Embedded PDF pages, reference rasters, 3D meshes
```

**manifest.json:**
```json
{
  "version": "0.1.0",
  "engine": "evcs",
  "compatible_api": ">=0.1.0",
  "created": "2026-01-15T10:30:00Z",
  "description": "Engineering Vector Canvas System project"
}
```

**scene.json** (serialized `Scene` with serde):
- Layer hierarchy
- Active camera transform
- Selected items, constraints

**strokes.bin** (binary, bincode-serialized `Vec<Stroke>`):
- Flat memory layout for high-performance access
- Each stroke: `[id: u64, layer_id: u64, color: f64×4, width: f64, points_count: u32, points: f64×(count×2)]`

### 5. iced Application (`app.rs`)

The iced app structure:

```rust
fn main() -> iced::Result {
    App::run(app::Settings::default())
}
```

Key widgets:
- **Canvas widget**: `iced::widget::canvas` with wgpu backend for 2D vector rendering
- **Pan/zoom**: Middle-drag to pan, mouse wheel to zoom, Ctrl+Zoom or double-click reset
- **Grid overlay**: Optional grid with 4px baseline (per design spec)
- **Coordinate readout**: Status bar shows world coordinates under cursor

### 6. MCP Server Integration (`mcp.rs`)

Initial MCP support for agent integration:

```rust
async fn run(project: Option<String>) {
    // JSON-RPC over stdio
    // Initial tools:
    // - evcs_get_scene_summary: returns layer count, bounding box, Git status
    // - evcs_create_vector_stroke: injects a vector path onto specified layer
}
```

## Verification Criteria (Phase 0)

- ✅ Window opens with `iced` at 60+ FPS on target hardware
- ✅ Mouse wheel zooms, middle-drag pans the canvas
- ✅ `Ctrl+0` or double-click resets viewport to origin
- ✅ `.evcs` archive can be opened and parsed correctly
- ✅ Roundtrip: save a scene with 1000+ strokes, reopen it, verify all strokes present
- ✅ MCP server starts and responds to `evcs_get_scene_summary` with valid JSON
- ✅ Color palette matches spec: `#F8F9FA` background, `#1A1A1A` primary text/strokes, `#0066FF` accent

## Dependencies (Cargo.toml)

Confirmed for Phase 0:
- `iced = "0.14"` with `["image", "tokio"]` features
- `iced_wgpu` (canvas widget backend)
- `tokio = "1"` with `["full"]`
- `clap = "4"` with `["derive"]`
- `serde` + `serde_json` for serialization
- `bincode` for stroke binary buffers
- `rstar` = "0.13" for R-Tree spatial indexing
- `git2` = "0.19" for native Git operations
- `zip` = "4" with `["deflate"]` for .evcs container
- `walkdir` = "2" for directory traversal
- `chrono` = "0.4" with `["serde"]` for timestamps
- `image` = "0.25" with `["png", "jpeg", "webp", "gif"]` for raster references