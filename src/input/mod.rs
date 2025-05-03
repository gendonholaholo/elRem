use anyhow::{bail, Result};
// use std::fs;
use std::path::{Path, PathBuf};
use colored::*;
use walkdir::WalkDir;

// Konstanta untuk ekstensi yang diizinkan, hanya visible di dalam crate
pub(crate) const ALLOWED_EXTENSIONS: [&str; 3] = ["png", "jpg", "jpeg"];

/// Memeriksa apakah ekstensi file didukung.
fn is_valid_image_extension(path: &Path) -> bool {
    path.extension()
        .and_then(|os_str| os_str.to_str())
        .map(|s| s.to_lowercase())
        .map_or(false, |ext| ALLOWED_EXTENSIONS.contains(&ext.as_str()))
}

/// Memvalidasi apakah path adalah file gambar yang didukung.
/// Mengembalikan Ok(vec![path]) jika valid, sebaliknya Err.
pub fn validate_single_file(path: &Path) -> Result<Vec<PathBuf>> {
    if !path.is_file() {
        bail!("Input path bukan file: {:?}", path);
    }
    if is_valid_image_extension(path) {
        Ok(vec![path.to_path_buf()])
    } else {
        let ext_str = path.extension().and_then(|s| s.to_str()).unwrap_or("tidak ada");
        bail!(
            "Format file tidak didukung ('{}') atau file tidak berekstensi: {:?}",
            ext_str,
            path
        )
    }
}

/// Mengumpulkan semua file gambar yang didukung dari sebuah direktori secara rekursif.
pub fn collect_images_from_dir(dir_path: &Path) -> Result<Vec<PathBuf>> {
    let mut image_files = Vec::new();
    for entry_result in WalkDir::new(dir_path).into_iter() {
        match entry_result {
            Ok(entry) => {
                if entry.file_type().is_file() {
                    let path = entry.path();
                    if is_valid_image_extension(path) {
                        image_files.push(path.to_path_buf());
                    }
                }
            }
            Err(err) => {
                // Tetap tampilkan warning jika ada error akses direktori/file
                eprintln!(
                    "{}",
                    format!("Warning: Gagal mengakses entry: {}", err).yellow()
                );
            }
        }
    }
    Ok(image_files)
} 