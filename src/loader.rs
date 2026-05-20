use anyhow::{Context, Result};
use std::path::Path;
use walkdir::WalkDir;

use crate::viewer::ImageInfo;

/// Supported image formats.
const SUPPORTED_EXTENSIONS: &[&str] = &["png", "jpg", "jpeg"];

/// Maximum memory budget for decoded images (500 MB).
/// Dimensions are checked before full decode — if width × height × 4 > max,
/// the image is rejected.
const MAX_DECODE_BYTES: u64 = 500 * 1024 * 1024;

/// Scan a directory (recursively) for supported image files.
/// Returns paths sorted alphabetically.
pub fn scan_directory(root: &str) -> Result<Vec<std::path::PathBuf>> {
    let root = Path::new(root);

    let (files, is_file): (Vec<_>, bool) = if root.is_file() {
        (vec![root.to_path_buf()], true)
    } else if root.is_dir() {
        let mut files: Vec<_> = WalkDir::new(root)
            .follow_links(false) // No symlink following — security
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| {
                e.path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| SUPPORTED_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
                    .unwrap_or(false)
            })
            .map(|e| e.path().to_path_buf())
            .collect();
        files.sort();
        (files, false)
    } else {
        anyhow::bail!("Path does not exist: {}", root.display());
    };

    if files.is_empty() && is_file {
        // Single file that doesn't match — still accept it, let decode fail with a
        // proper error
        return Ok(vec![root.to_path_buf()]);
    }

    Ok(files)
}

/// Decode an image from disk, with memory budget enforcement.
///
/// # Safety
/// Dimensions are checked before full allocation. If the image would exceed
/// `max_bytes`, it is rejected before decoding the pixel data.
pub fn decode_image(
    path: &Path,
    max_bytes: u64,
) -> Result<(image::DynamicImage, ImageInfo)> {
    // Peek dimensions first (opens the file, reads header, closes)
    let (width, height) = image::ImageReader::open(path)
        .with_context(|| format!("Failed to open '{}'", path.display()))?
        .with_guessed_format()
        .with_context(|| format!("Unknown image format: '{}'", path.display()))?
        .into_dimensions()
        .with_context(|| format!("Failed to read dimensions from '{}'", path.display()))?;

    // Memory budget guardrail
    let estimated_bytes = width as u64 * height as u64 * 4; // RGBA
    if estimated_bytes > max_bytes {
        return Err(anyhow::anyhow!(
            "Image too large: {}×{} = {} MB (max {} MB). Skipping '{}'",
            width,
            height,
            estimated_bytes / (1024 * 1024),
            max_bytes / (1024 * 1024),
            path.display(),
        ));
    }

    // Re-open and decode (ImageReader consumes itself on into_dimensions)
    let image = image::ImageReader::open(path)
        .with_context(|| format!("Failed to re-open '{}'", path.display()))?
        .with_guessed_format()
        .with_context(|| format!("Unknown image format: '{}'", path.display()))?
        .decode()
        .with_context(|| format!("Failed to decode '{}' — file may be corrupted", path.display()))?;

    let color_type = image.color();
    let format = match color_type {
        image::ColorType::Rgb8 | image::ColorType::Rgba8 => "RGB",
        image::ColorType::L8 | image::ColorType::La8 => "Grayscale",
        _ => "Other",
    };

    let file_size = std::fs::metadata(path)
        .map(|m| m.len())
        .unwrap_or(0);

    let info = ImageInfo {
        width,
        height,
        format: format.to_string(),
        size_mb: file_size as f64 / (1024.0 * 1024.0),
    };

    Ok((image, info))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_nonexistent_path() {
        assert!(scan_directory("/tmp/pictura_nonexistent_test_dir").is_err());
    }

    #[test]
    fn test_memory_budget_rejects_huge_image() {
        // An image claiming 20000×20000 would need ~1.6GB
        // Our budget is 500MB — should reject before decode
        let path = Path::new("tests/fixtures/valid.png");
        // This tests the budget logic, not a real huge image
        let result = decode_image(path, 1); // 1 byte budget — everything fails
        assert!(result.is_err());
    }
}
