//! EVCS Archive Format
//! 
//! The `.evcs` ZIP container housing the scene state and stroke data.
//! 
//! Structure:
//! 
//! my_project.evcs (ZIP Archive)
//! ├── manifest.json       Version, metadata, engine compatibility
//! ├── scene.json          Layer hierarchy, active constraints, camera vectors
//! ├── strokes.bin         Binary packed raw/smoothed point data
//! └── assets/             Embedded PDF pages, reference rasters, 3D meshes

use serde_json::Value;
use std::path::Path;
use zip::{ZipArchive, ZipWriter};
use std::io::{BufReader, Cursor, Write as IoWrite};
use bincode;

/// Represents the full `.evcs` document, combining metadata, scene graph,
/// and serialized stroke data for persistence or network transfer.
#[derive(Debug, Clone)]
pub struct Document {
    /// Archive metadata (version, engine compatibility, timestamps).
    pub manifest: Value,
    /// The complete scene graph including layers, camera, and strokes.
    pub scene: crate::engine::scene::Scene,
    /// All strokes in the scene, binary-packed for high-performance access.
    pub strokes: Vec<crate::engine::scene::Stroke>,
}

/// Open and parse a `.evcs` archive ZIP file.
/// 
/// Reads the following entries from the archive:
/// - `manifest.json` → parsed as `serde_json::Value`
/// - `scene.json` → deserialized as `crate::engine::scene::Scene`
/// - `strokes.bin` → bincode-deserialized as `Vec<crate::engine::scene::Stroke>`
pub fn open_archive(path: &Path) -> Result<Document, zip::error::ZipError> {
    let file = std::fs::File::open(path)?;
    let mut archive = ZipArchive::new(file)?;

    // Read manifest.json
    let manifest_file = archive.by_name("manifest.json")?;
    let manifest_reader = BufReader::new(manifest_file);
    let manifest: Value = serde_json::from_reader(manifest_reader)?;

    // Read scene.json
    let scene_file = archive.by_name("scene.json")?;
    let scene_reader = BufReader::new(scene_file);
    let scene: crate::engine::scene::Scene = serde_json::from_reader(scene_reader)?;

    // Read strokes.bin (bincode deserialized)
    let strokes_file = archive.by_name("strokes.bin")?;
    let strokes_data: Vec<u8> = strokes_file.bytes()
        .collect::<std::result::Result<Vec<u8>, _>>()?;
    let strokes: Vec<crate::engine::scene::Stroke> = bincode::deserialize(&strokes_data)?;

    Ok(Document { manifest, scene, strokes })
}

/// Save the current state to a `.evcs` archive ZIP file.
/// 
/// Writes the following entries into the ZIP:
/// - `manifest.json` — serde_json serialized
/// - `scene.json` — serde_json serialized Scene
/// - `strokes.bin` — bincode serialized Vec<Stroke>
pub fn save_archive(path: &Path, document: &Document) -> Result<(), zip::error::ZipError> {
    let file = std::fs::File::create(path)?;
    let mut writer = ZipWriter::new(file);

    // Write manifest.json
    let manifest_bytes = serde_json::to_string(&document.manifest)?;
    writer.append_data("manifest.json", manifest_bytes.as_bytes())?;

    // Write scene.json
    let scene_bytes = serde_json::to_string(&document.scene)?;
    writer.append_data("scene.json", scene_bytes.as_bytes())?;

    // Write strokes.bin (bincode serialized)
    let strokes_bytes = bincode::serialize(&document.strokes)?;
    writer.append_data("strokes.bin", &strokes_bytes)?;

    writer.finish()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempfile;

    #[test]
    fn test_open_nonexistent_archive_fails() {
        let result = open_archive(Path::new("/tmp/nonexistent.evcs"));
        assert!(result.is_err());
    }

    #[test]
    fn test_save_and_open_roundtrip() {
        // Create a document with a scene and strokes
        let mut scene = crate::engine::scene::Scene::new();
        let color = crate::engine::scene::Color::accent();
        let stroke = crate::engine::scene::Stroke::new(color, 2.0);
        scene.add_stroke(stroke);

        let mut doc = Document {
            manifest: serde_json::json!({
                "version": "0.1.0",
                "engine": "evcs",
                "created": "2026-01-15T10:30:00Z"
            }),
            scene,
            strokes: Vec::new(), // will be populated after save
        };

        // Save to a temp file
        let tmp = tempfile().unwrap();
        let path = tmp.path();

        let save_result = save_archive(path, &doc);
        assert!(save_result.is_ok(), "save_archive should succeed");

        // Reopen and verify
        let open_result = open_archive(path);
        assert!(open_result.is_ok(), "open_archive should succeed after save");
        let reopened = open_result.unwrap();

        assert_eq!(reopened.scene.stroke_count(), 1);
        assert!(reopened.strokes.len() >= 1);
    }
}