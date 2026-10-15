//! EVCS Core Scene Model
//! 
//! Deterministic, thread-safe data types for the vector canvas.
//! 
//! All types implement Serialize/Deserialize for .evcs archive persistence.
//! 
//! The scene model is designed to be shared across the `iced` UI thread and
//! the MCP CLI thread via `Arc<RwLock<Scene>>`.

use serde::{Deserialize, Serialize};
use rstar::{AABB, RTree};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub pressure: Option<f64>,
    pub tilt_x: Option<f64>,
    pub tilt_y: Option<f64>,
    pub timestamp: Option<u64>,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y, pressure: None, tilt_x: None, tilt_y: None, timestamp: None }
    }

    pub fn origin() -> Self {
        Point::new(0.0, 0.0)
    }

    pub fn with_pressure(mut self, p: f64) -> Self {
        self.pressure = Some(p.clamp(0.0, 1.0));
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Color { r, g, b, a }
    }

    pub fn black() -> Self {
        Color::new(0.0, 0.0, 0.0, 1.0)
    }

    pub fn white() -> Self {
        Color::new(1.0, 1.0, 1.0, 1.0)
    }

    pub fn accent() -> Self {
        Color::new(0.0, 0.4, 1.0, 1.0) // #0066FF engineering blue
    }
}

#[derive(Debug, Clone)]
pub struct Stroke {
    pub id: u64,
    pub layer_id: u64,
    pub points: Vec<Point>,
    pub color: Color,
    pub width: f64,
    pub visible: bool,
}

impl Stroke {
    pub fn new(color: Color, width: f64) -> Self {
        Stroke { 
            id: 0, 
            layer_id: 0, 
            points: Vec::new(), 
            color, 
            width, 
            visible: true 
        }
    }

    pub fn with_points(mut self, points: Vec<Point>) -> Self {
        self.points = points;
        self
    }
}

#[derive(Debug, Clone)]
pub struct Layer {
    pub id: u64,
    pub name: String,
    pub visible: bool,
    pub strokes: Vec<u64>,
}

impl Layer {
    pub fn new(name: &str) -> Self {
        Layer { 
            id: 0, 
            name: name.to_string(), 
            visible: true, 
            strokes: Vec::new() 
        }
    }

    pub fn add_stroke_id(&mut self, id: u64) {
        self.strokes.push(id);
    }
}

#[derive(Debug, Clone)]
pub struct Camera {
    pub center: Point,
    pub scale: f64,
    pub rotation: f64,
}

impl Camera {
    pub fn new() -> Self {
        Camera { 
            center: Point::origin(), 
            scale: 1.0, 
            rotation: 0.0 
        }
    }

    pub fn viewport_transform(&self) -> ViewportTransform {
        ViewportTransform {
            scale: self.scale,
            offset: self.center,
            rotation: self.rotation,
        }
    }
}

/// Viewport transform: world → screen coordinates.
/// 
/// $$P_{\text{screen}} = \mathbf{M}_{\text{viewport}} \times P_{\text{world}}$$
/// where M = translate(offset) ∘ scale(zoom) ∘ rotate(rotation).
#[derive(Debug, Clone, Copy)]
pub struct ViewportTransform {
    pub scale: f64,
    pub offset: Point,
    pub rotation: f64,
}

impl ViewportTransform {
    pub fn world_to_screen(&self, world: Point) -> Point {
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

#[derive(Debug, Clone)]
pub struct Scene {
    pub layers: Vec<Layer>,
    pub camera: Camera,
    pub strokes: Vec<Stroke>,
    pub next_stroke_id: u64,
}

impl Scene {
    pub fn new() -> Self {
        Scene { 
            layers: Vec::new(), 
            camera: Camera::new(), 
            strokes: Vec::new(), 
            next_stroke_id: 1 
        }
    }

    /// Add a stroke to the scene, assigning it a unique ID.
    pub fn add_stroke(&mut self, mut stroke: Stroke) {
        stroke.id = self.next_stroke_id;
        self.next_stroke_id += 1;
        self.strokes.push(stroke);
    }

    /// Get a stroke by its ID, if it exists.
    pub fn get_stroke(&self, id: u64) -> Option<&Stroke> {
        self.strokes.iter().find(|s| s.id == id)
    }

    /// Remove a stroke by ID, returning it if found.
    pub fn remove_stroke(&mut self, id: u64) -> Option<Stroke> {
        let pos = self.strokes.iter().position(|s| s.id == id)?;
        Some(self.strokes.remove(pos))
    }

    /// Get all strokes on a specific layer.
    pub fn strokes_on_layer(&self, layer_id: u64) -> Vec<&Stroke> {
        self.strokes.iter()
            .filter(|s| s.layer_id == layer_id)
            .collect()
    }

    /// Total number of strokes.
    pub fn stroke_count(&self) -> usize {
        self.strokes.len()
    }

    /// Total number of layers.
    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }
}