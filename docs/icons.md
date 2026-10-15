### Option 2: Custom TTF Icon Font via `verglas` (Best Performance / Zero File I/O)

If you need dozens of tiny UI icons (like snapping toggles, layers, zoom controls, tool selectors) rendered instantly without hitting disk I/O for individual files, compile an SVG icon set into a custom **TrueType Font (`.ttf`)** using pure Rust.

* **The Crate:** [`verglas`](https://crates.io/crates/verglas)
* **How it works:** You point it to a folder of your favorite SVG icons during build time (`build.rs`), and it automatically packages them into a lightweight `.ttf` font file. You then load the font into `iced` and render icons as text glyphs using standard text formatting.
* **Why it fits engineers:** Extremely fast layout calculations, tiny memory footprint, and trivial color/size modifications dynamically inside code using your theme tokens (`#1A1A1A` or `#0066FF`).

---

### Suggested Icon Selection for EVCS

To match the **Concepts.app + CAD** workflow specified in your architecture, download or bundle vectors covering these specific domains:

* **Tools:** Pen, Vector Path/Node, Eraser/Slice, Nudge, Shape Guide 
(Rectangle/Circle/Arc), Text, Ruler/Dimension. * **Canvas Controls:** 
Zoom In/Out, Fit-to-Screen, Grid Snap toggles, Isometric View, Layer 
Stack. * **Collaboration & Git:** Cloud/Cloudflared Tunnel status 
indicator, Git Branch/Commit nodes, Peer Connection dot.
we will be using lucid icons, am still getting it
