# EVCS: Iced UI, Native Agent CLI, and MCP Architecture

This document specifies the technical architecture and phase guide for implementing the **Engineering Vector Canvas System (EVCS)** using **Rust**, the **`iced`** GUI framework, and a native **Model Context Protocol (MCP)** command-line interface (CLI) to empower AI coding agents and automated pipelines.

---

## 1. Architectural Blueprint

The EVCS agent-native architecture separates the application into three decoupled layers:

1. **The Core Engine & Storage**: High-performance Rust vector backend (R-Tree spatial indexing, wgpu rasterizer/canvas renderer, `.evcs` ZIP containerization, and embedded Git version control via `git2-rs`).
2. **The `iced` Desktop Frontend**: Immediate/Retained hybrid UI using `iced`, styled with a minimalist monochrome aesthetic (`#F8F9FA` off-white, `#1A1A1A` charcoal, `#0066FF` precision blue).
3. **The Embedded MCP CLI Layer**: A native stdio/HTTP Model Context Protocol server bundled into the primary executable binary, enabling LLM agents to query the canvas state, execute drawing/CAD commands, inspect Git histories, and automate exports.

```
+---------------------------------------------------------------------------------+
|                                    User / AI Agent                              |
+-----------------------------------------+---------------------------------------+
                                          |
                        +-----------------+-----------------+
                        |                                   |
                        v                                   v
        +-------------------------------+   +-------------------------------+
        |     iced Desktop Frontend     |   |   Embedded MCP Server (CLI)   |
        |  - Infinite Canvas Viewport   |   |   - JSON-RPC over Stdio       |
        |  - Tool Wheel & Panels        |   |   - Scene Graph Tool Hooks    |
        +---------------+---------------+   +---------------+---------------+
                        |                                   |
                        +-----------------+-----------------+
                                          |
                                          v
                    +-------------------------------------------+
                    |           EVCS Core Rust Engine           |
                    |  - R-Tree Spatial Index                   |
                    |  - wgpu Renderer & Vector Rasterizer      |
                    |  - .evcs ZIP Manager & libgit2 Integration|
                    +-------------------------------------------+

```

---

## 2. Design System & Minimalist Aesthetics

The UI follows a strict, high-contrast, clean-room professional palette tailored for engineers.

| Design Token | Hex Code | Visual Target | Component Application |
| --- | --- | --- | --- |
| **Canvas Background** | `#F8F9FA` | Off-White / Porcelain | Infinite drawing board workspace background |
| **Panel Surface** | `#EDEDED` | Soft Grey | Floating tool palettes, sidebars, header bars |
| **Active Border / Interactive** | `#E0E0E0` | Medium Grey | Element boundaries, divider lines, disabled items |
| **Primary Text / Strokes** | `#1A1A1A` | Charcoal Black | Vector paths, primary labels, window titles |
| **Secondary Metadata** | `#555555` | Muted Dark Grey | Coordinate values, dimension readouts, hints |
| **Engineering Accent** | `#0066FF` | Precision Blue | Selected nodes, active tools, grid snap markers |

---

## 3. Native Agent Architecture & MCP CLI Specification

To ensure agents can interact natively with EVCS, the application compiles into a dual-mode binary:

* **GUI Mode** (default): Launches the `iced` desktop window.
* **CLI / MCP Mode** (`evcs-cli --mcp`): Exposes a standard Model Context Protocol server over `stdio`, allowing local LLMs (like Claude Desktop, Cursor, or custom agents) to manipulate the workspace directly.

### Supported MCP Tool Declarations

The embedded MCP server registers the following core tool definitions:

1. **`evcs_get_scene_summary`**: Retrieves structural metadata, layer count, bounding box coordinates, and Git status of the active project.
2. **`evcs_create_vector_stroke`**: Injects a custom vector path (array of points with pressure attributes) onto a specified layer.
3. **`evcs_add_3d_primitive`**: Generates a 3D CAD primitive (box, cylinder, extrusion) with explicit dimension constraints.
4. **`evcs_export_document`**: Renders and exports the current workspace state to SVG, DXF, PDF, or PNG.
5. **`evcs_git_commit`**: Stages workspace changes and commits them with an automated or provided message string.

---

## 4. Phase Implementation Guide

---

### Phase 0: Workspace Bootstrap & `iced` Window Core

Establish the Rust workspace configuration, integrate `iced` with a custom wgpu canvas element, and set up the `.evcs` archive schema.

```
evcs/
├── Cargo.toml
├── src/
│   ├── main.rs            # Entry point (CLI vs GUI detector)
│   ├── app.rs             # iced application state loop
│   ├── canvas/            # wgpu canvas rendering pipeline
│   ├── engine/            # R-Tree, spatial structures, and scene model
│   ├── mcp/               # Model Context Protocol JSON-RPC server implementation
│   └── git/               # libgit2 wrapper bindings

```

#### Key Objectives

1. Configure `Cargo.toml` with `iced`, `wgpu`, `tokio`, `git2`, and `serde`.
2. Implement the root application structure in `iced` handling window resizing, mouse/stylus events, and pan/zoom transform matrices.
3. Initialize the `.evcs` ZIP container structure (`manifest.json`, `scene.json`, `strokes.bin`).

---

### Phase 1: Vector Canvas Engine & `iced` Integration

Implement core stroke capture, Catmull-Rom smoothing, R-Tree spatial indexing, and the minimalist monochrome UI panels.

#### Key Objectives

1. **Interactive Canvas Widget:** Build a custom `iced::widget::Renderer` integration backed by `wgpu` for high-FPS stroke rendering.
2. **Tool Wheel & Floating Menus:** Create layout components matching the monochrome design spec (`#F8F9FA` / `#1A1A1A` / `#0066FF`).
3. **Spatial R-Tree Indexing:** Implement stroke lookup, collision detection, and selection highlighting using `rstar`.

---

### Phase 2: 3D CAD Primitives & Constraint Engine

Extend the drawing canvas to support 3D objects, parametric extrusions, and numerical dimension locks.

#### Key Objectives

1. **3D Viewport Layer:** Embed a 3D perspective camera viewport within the `iced` layout.
2. **Parametric Primitives:** Add support for boxes, cylinders, and 2D-to-3D vector profile extrusions.
3. **Constraint Manager:** Enforce numeric length, width, and height constraints with live visual feedback.

---

### Phase 3: Native MCP CLI & Agent Automation

Wire up the standard I/O Model Context Protocol server so that external AI coding assistants can inspect and modify canvas projects.

```rust
// Example conceptual MCP request routing loop
async fn handle_mcp_request(req: JsonRpcRequest) -> JsonRpcResponse {
    match req.method.as_str() {
        "evcs_get_scene_summary" => {
            // Read active scene graph and return JSON summary
        },
        "evcs_create_vector_stroke" => {
            // Parse stroke coordinates and mutate scene graph safely
        },
        _ => JsonRpcResponse::error(-32601, "Method not found".into())
    }
}

```

#### Key Objectives

1. **JSON-RPC Stdio Transport:** Implement asynchronous stdin/stdout listening using `tokio` to handle MCP handshake and tool execution commands.
2. **State Mutator Bridge:** Connect MCP tool handlers directly to the thread-safe application state model (`Arc<Mutex<SceneState>>`), ensuring atomic updates between AI agent actions and the active `iced` UI canvas.
3. **CLI Argument Parser:** Support flags like `--mcp` for headless agent integration and `--export <format>` for batch pipelines.

---

### Phase 4: Git Version Control & Production Polish

Integrate native repository management and optimize memory consumption for low-end hardware.

#### Key Objectives

1. **libgit2 Integration:** Enable local snapshot commits, branch switching, and remote sync directly from the sidebar project browser.
2. **Performance Tuning:** Profile frame times on integrated graphics to guarantee stable 60–120 FPS rendering during complex vector operations.

---