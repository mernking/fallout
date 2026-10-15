# Engineering Vector Canvas System (EVCS)

**Architectural Blueprint & Phase Implementation Guide**

---

## 1. System Vision & Technical Philosophy

The **Engineering Vector Canvas System (EVCS)** is a clean-room, cross-platform, infinite-canvas engineering ideation tool written entirely in **Rust**. It combines the fluid vector-raster sketching capabilities of applications like Concepts with precise 2D parametric geometry, dimensioning constraints, light 3D spatial modeling, local-first real-time collaboration, and native Git version control.

### Core Architectural Principles

* **Pure Rust Ecosystem:** Built entirely in Rust using `egui` and `wgpu` for zero-overhead, ultra-low-memory footprint, and deterministic 60–120 FPS performance on low-end hardware (e.g., Intel i5-4210U with integrated graphics).
* **Local-First & Sovereign:** No central cloud required. All state is stored in version-controlled `.evcs` ZIP archives, synced directly peer-to-peer via WebSockets with outbound `cloudflared` tunneling or local network discovery (mDNS).
* **Hybrid Vector Engine:** Retained-mode spatial indexing (R-Tree) for infinite-canvas rendering, editable vector paths with real-time pressure/tilt sampling, live curve smoothing (Chaikin / Catmull-Rom), and resolution-independent stroke serialization.
* **Minimalist Engineering Aesthetics:** A sharp, distraction-free monochrome interface designed for high information density, high contrast, and minimal visual noise.

---

## 2. Design System & UI Specification

The visual interface uses a strict three-tone design palette with high accessibility and sharp boundaries, tailored for desktop devices with stylus or precision mouse inputs.

```
+-------------------------------------------------------------------------------+
| [Project Browser: Archival List]  | [Tool Wheel]      [Canvas: Infinite Space] |
|-----------------------------------|  (Floating)                               |
| [>] Core Mechanics / Floor Plan   |     O  <- Ring    +----------------------+ |
| [v] Mechanical Bracket v2         |   O   O           |                      | |
|     * main.evcs [Modified]        |     O             |   2D Stroke / 3D     | |
|     * 3d_mesh.obj                 |               (0,0)                      | |
|-----------------------------------|                   +----------------------+ |
| [Git: main] [Collab: Peer Active] |                   | Rulers / Dimension   | |
+-------------------------------------------------------------------------------+

```

### Color Palette Specification

| Token | Hex Code | Visual Target | Application Area |
| --- | --- | --- | --- |
| **Surface Background** | `#F8F9FA` | Off-White / Porcelain | Infinite canvas backdrop, main window surface |
| **Surface Elevation 1** | `#EDEDED` | Soft Grey | Collapsible panels, project browser rows, tool wheel rings |
| **Surface Elevation 2** | `#E0E0E0` | Medium Grey | Modals, active hover states, grid overlays |
| **Primary Text / Line** | `#1A1A1A` | Charcoal Black | Primary vector strokes, text, wireframe edges, primary UI labels |
| **Secondary Text** | `#555555` | Muted Dark Grey | Subtitles, secondary metadata, unit dimensions |
| **Accent Primary** | `#0066FF` | Precision Engineering Blue | Active tool indicators, selected vector nodes, snapping alignment lines |
| **Accent Hover / Glow** | `#3385FF` | Soft Blue | Cursor hovers, live constraint preview lines |
| **Git & Collab Status** | `#10B981` | Emerald Green | Live peer connection state, committed status indicator |

### Typography & Spacing Rules

* **UI Typography:** Monospaced (`JetBrains Mono` or system default mono) for precision coordinate entries, dimensions, and panel readouts; Sans-Serif (`Inter` or system neutral) for menus.
* **Layout Grid:** Dense 4px baseline grid for compact floating tool overlays, collapsible sidebar drawers, and contextual radial menus.

---

## 3. High-Level System Architecture

```
                                  +---------------------------------------+
                                  |         User Interface (egui)         |
                                  |   [Project Browser] [Tool Wheel]     |
                                  +-------------------+-------------------+
                                                      |
                                                      v
                                  +---------------------------------------+
                                  |        App Core State & Commands      |
                                  +---------+-------------------+---------+
                                            |                   |
            +-------------------------------+                   +-------------------------------+
            |                                                                                   |
            v                                                                                   v
+-----------------------+                                                       +-----------------------+
|  Vector & 3D Engine   |                                                       | Multi-User & Version  |
|  (wgpu / R-Tree)      |                                                       | Management Subsystems |
+-----------+-----------+                                                       +-----------+-----------+
            |                                                                                   |
   +--------+--------+                                                                 +--------+--------+
   |                 |                                                                 |                 |
   v                 v                                                                 v                 v
+--------+      +--------+                                                        +--------+        +--------+
| Vector |      | 3D CSG |                                                        | Local  |        | P2P    |
| Paths  |      | Engine |                                                        | Git    |        | Collab |
+--------+      +--------+                                                        +--------+        +--------+

```

---

## 4. Phase-by-Phase Development Roadmap

---

### Phase 0: System Foundation & Core Rendering Architecture

Establish the platform infrastructure, native windowing, spatial acceleration structures, and file system primitives.

#### Key Objectives

1. **Windowing & Graphics Pipeline Setup:** Initialize `wgpu` backend targeting Vulkan/Metal/DirectX 12 with software fallback; integrate `egui` as the primary immediate-mode UI shell.
2. **Infinite Canvas Math Engine:** Build a matrix transform engine for Pan, Zoom, and Rotation supporting viewport transformations:

$$\mathbf{P}_{\text{screen}} = \mathbf{M}_{\text{viewport}} \times \mathbf{P}_{\text{world}}$$


3. **Raw Stylus Input Handler:** Bind native OS platform input events (Windows Ink, x11/wayland libinput, macOS Cocoa) to collect pressure ($p \in [0.0, 1.0]$), tilt ($\theta, \phi$), and high-frequency sample timestamps.
4. **Data Engine (.evcs format):** Define the ZIP container format housing `scene.json` (scene graph & layers), `strokes.bin` (flat memory stroke buffers), and `manifest.json`.

```
my_project.evcs (ZIP Archive)
├── manifest.json       # Version, metadata, engine compatibility
├── scene.json          # Layer hierarchy, active constraints, camera vectors
├── strokes.bin         # Binary packed raw/smoothed point data
└── assets/             # Embedded PDF pages, reference rasters, 3D meshes

```

#### Verification Criteria

* Infinite canvas panning and zooming maintaining continuous 120 FPS on target Intel i5 integrated graphics.
* Binary serialize and deserialize roundtrips for 50,000 random points in under 15ms.

---

### Phase 1: Core Concepts Vector Engine & Precision Tools

Implement stroke capture, live smoothing, editable vector geometry, precision snapping, and grid overlays.

```
   Raw Pointer Input Stream (x, y, pressure, tilt)
                         │
                         ▼
        Catmull-Rom Curve Fitting Engine
                         │
                         ▼
        Dynamic Width Tessellation (wgpu)
                         │
                         ▼
        R-Tree Spatial Index (Culling & Selection)
                         │
                         ▼
             Immediate Canvas Output

```

#### Key Objectives

1. **Stroke Capture & Live Smoothing:** Implement Catmull-Rom / Chaikin curve fitting to refine raw stylus input data into smooth parametric splines in real time.
2. **Dynamic Mesh Generation:** Generate vector stroke meshes dynamically on the GPU based on brush type (e.g., Fixed-Wire, Pressure-Pen, Soft-Marker) with pressure-sensitive width calculation:

$$w(p) = w_{\text{base}} \cdot p^{\gamma}$$


3. **Spatial Acceleration Index (R-Tree):** Index all vector strokes using a 2D bounding box R-Tree (`rstar` crate) to support sub-millisecond frustum culling and spatial point queries (vector selection, slicing).
4. **Tool System Implementation:**
* **Floating Radial Tool Wheel:** Multi-ring menu rendered using egui layers with configurable slots for colors, opacity, and brush presets.
* **Vector Eraser / Slice:** Geometry cleavage tool that splits a single vector path into two discrete stroke objects at the intersection points.
* **Nudge / Sculpt:** Pull vector nodes within an interactive influence radius without destroying path continuity.


5. **Precision & Measurement Layer:**
* **Grids:** Imperial, Metric, Isometric, and Perspective grids with live coordinate readouts.
* **Snapping Engine:** Snap to grid intersections, vector stroke control points, tangents, midpoints, and perpendiculars.
* **Associative Dimensions:** Dynamic dimension labels attached to strokes that update length/angle values live as geometry changes.



#### Verification Criteria

* Sub-10ms rendering time for scene graphs containing over 20,000 distinct vector strokes.
* Vector slicing correctly splits complex paths into valid child paths without node corruption.

---

### Phase 2: 3D Spatial Canvas & Light Parametric Modeling

Extend the 2D infinite canvas into 3D space, supporting parametric primitives, extrusions, and dimensional constraint enforcement.

```
       2D Vector Sketch Path
                 │
                 ▼
     Parametric Extrusion (CSG)
                 │
                 ▼
      Dimension Constraint Engine
      (Width, Length, Angle Locks)
                 │
                 ▼
   wgpu 3D Viewport Mesh Rendering

```

#### Key Objectives

1. **Hybrid 3D Viewport Engine:** Implement a 3D scene node layer inside the canvas using `wgpu` depth buffers and perspective cameras alongside the 2D canvas pipeline.
2. **Parametric Primitives Generator:**
* Basic Primitives: Boxes, Cylinders, Spheres, Cones, Toruses.
* Extrusion: Convert closed 2D vector sketches into 3D extruded solids along a normal vector with customizable depth constraints.


3. **3D Constraint & Dimensioning Engine:**
* Allow numerical input locks for length ($L$), width ($W$), height ($H$), and rotational angles ($\theta$).
* Interactive ruler handles attached to 3D bounding boxes with real-time numeric readouts in metric or imperial units.


4. **3D Model Asset Support:** Parse and export STL, OBJ, and basic STEP mesh files directly into/from the `.evcs` container.

#### Verification Criteria

* Extrude closed 2D vector loops into 3D meshes instantly while maintaining editable linkage to the source 2D shape.
* Stable 3D wireframe and shaded rendering alongside 2D vector stroke layers without z-fighting or context switches.

---

### Phase 3: Peer-to-Peer Real-Time Collaboration & Cloudflared Security

Implement direct local-first synchronization for multi-user real-time canvas editing without a centralized server.

```
[Peer A: EVCS Node] <--- WebSocket CRDT Delta ---> [Peer B: EVCS Node]
        │                                                  │
        └─── via cloudflared Outbound Tunnel / LAN mDNS ───┘

```

#### Key Objectives

1. **CRDT Delta Engine:** Implement a Conflict-Free Replicated Data Type (CRDT) operational model for scene graph mutations:
* **Strokes:** Immutable event stream with tombstone markers for deletions.
* **Positions & Transforms:** Last-Write-Wins (LWW) Register with logical vector clocks.


2. **WebSocket Networking Core:**
* **LAN Discovery:** Automatic peer detection using mDNS over local networks.
* **Tunneling (`cloudflared`):** Native wrapper around the `cloudflared` binary to generate secure outbound tunnels, returning shareable join URLs without requiring open incoming ports or port forwarding.


3. **Multi-User Presence Layer:** Display active collaborator cursors with custom usernames, selected tool paths, and real-time bounding box selections.

#### Verification Criteria

* Multi-user state reconciliation resolves concurrently generated strokes from 4 peers without data loss or desynchronization.
* Local peer discovery and connection establish over LAN within 2 seconds.

---

### Phase 4: Embedded Version Control (Git Integration) & Archive Browser

Integrate Git natively into the project lifecycle to allow branches, history tracking, and visual project organization.

```
+-----------------------------------------------------------------------------------+
| Side Project Browser                                                              |
|-----------------------------------------------------------------------------------|
| [New Canvas] [Import...]                                                          |
|                                                                                   |
| v ARCHIVE: /home/user/engineering_projects                                        |
|   v [Project] Gearbox Redesign                                                    |
|     * main.evcs  [Git: 3 commits ahead] [Syncing...]                              |
|     * housing_sketch.evcs                                                         |
|   v [Project] Structural Truss Analysis                                           |
|     * truss_v1.evcs                                                               |
|                                                                                   |
|-----------------------------------------------------------------------------------|
| Git Controls: [Commit]  [Sync/Push]  [Branch: main v]                             |
+-----------------------------------------------------------------------------------+

```

#### Key Objectives

1. **Native Git Binding Engine:** Embed `libgit2` (via `git2-rs`) to handle local repository creation, committing, branching, and remote syncs directly inside the workspace directory.
2. **Auto-Unpack & Archive Syncing:** Unpack `.evcs` ZIP archives into a temporary working directory upon opening, tracking raw text `.json` files and binary stroke buffers using Git for minimal delta storage sizes.
3. **Visual Side Project Browser:**
* Render an archive-style file tree showing projects, folders, and `.evcs` canvas files.
* Display real-time Git status badges (Modified, Untracked, Ahead/Behind, Syncing).
* Visual commit history browser displaying historical snapshots with instant rollbacks.



#### Verification Criteria

* Executing Git commit, branch creation, and checkout operations from within the app takes under 100ms.
* File browser accurately reflects directory updates, Git state changes, and workspace additions in real time.

---

### Phase 5: Production Hardening, Pipeline Exports, & Optimization

Refine low-end execution performance, polish multi-format engineering exports, and lock down system stability.

#### Key Objectives

1. **Low-End Hardware Optimization:**
* Implement region-based dirty canvas repainting to minimize GPU cycles when the canvas is stationary.
* Optimize memory allocation pools to limit idle RAM consumption below 120 MB.


2. **Industrial Vector & CAD Exports:**
* **SVG:** Export layered, clean vector paths with grouped structures and styled attributes.
* **DXF:** Generate standard AutoCAD-compatible DXF files preserving scale, layers, and geometry types.
* **PDF:** Export multi-page vector drawings with embedded fonts and accurate real-world scale factors.
* **PNG/PSD:** High-DPI rasterization with layer preserving flags.


3. **Batch Export CLI:** Include a headless command-line interface executable mode (`evcs-cli`) to enable automated export pipelines in CI/CD environments:
```bash
evcs-cli export --format dxf --scale 1:50 input_project.evcs -o output_plan.dxf

```



#### Verification Criteria

* Cold startup time remains under 1.5 seconds on target hardware.
* Exported DXF and SVG files open correctly in third-party CAD applications (e.g., AutoCAD, FreeCAD, Inkscape) without missing paths or scale distortions.

---

## 5. Technology Stack Reference

| Component | Library / Crate | Justification |
| --- | --- | --- |
| **Language** | `Rust (2024 Edition)` | Maximum memory safety, native hardware performance, cross-platform compilation. |
| **GUI Framework** | `egui` / `eframe` | Lightweight immediate-mode interface with simple custom theming and minimal overhead. |
| **Graphics API** | `wgpu` | Modern cross-platform GPU API abstracting Vulkan, Metal, and DirectX 12. |
| **Spatial Indexing** | `rstar` | High-performance 2D/3D R-Tree structure for real-time canvas frustum culling. |
| **Git Backend** | `git2-rs` | Direct C-bindings to `libgit2` for fast, native Git operations without shell overhead. |
| **Networking** | `tokio` + `tungstenite` | Asynchronous I/O core for peer-to-peer WebSocket communications. |
| **Serialization** | `serde` + `bincode` | Compact binary serialization for high-frequency networking and storage payloads. |

---

## 6. Verification & Implementation Strategy

To begin execution of this project, build each module iteratively, starting with **Phase 0**.

### Immediate Development Steps

1. Initialize the base Rust repository configured with `eframe` (`egui` + `wgpu`).
2. Construct the core infinite-canvas matrix transformation layer to verify pan/zoom controls.
3. Implement the primary monochrome palette rules (`#F8F9FA`, `#1A1A1A`, `#0066FF`) across basic window elements to establish visual consistency early.