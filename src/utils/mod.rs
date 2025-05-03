use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Memeriksa apakah sebuah perintah eksternal ada di PATH sistem.
/// Menjalankan `cmd --version` dan memeriksa status sukses.
/// Menyembunyikan output stdout/stderr dari perintah cek.
pub fn check_command_exists(cmd: &str) -> Result<bool> {
    Command::new(cmd)
        .arg("--version") // Asumsi umum, mungkin perlu penyesuaian untuk cmd langka
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .or_else(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                Ok(false) // Perintah tidak ditemukan
            } else {
                // Error lain saat mencoba menjalankan (permission, etc.)
                Err(e).context(format!("Gagal menjalankan check untuk perintah: {}", cmd))
            }
        })
}

/// Menentukan path output berdasarkan input asli (file/dir) dan path file saat ini.
/// Untuk direktori, membuat subfolder `elrem_output`.
/// Untuk file, menambahkan suffix `-nobg`.
pub fn determine_output_path(
    current_file: &Path,
    is_original_input_dir: bool,
    original_input_path: &Path,
) -> Result<PathBuf> {
    if is_original_input_dir {
        // Kasus Input Direktori:
        let relative_path = current_file
            .strip_prefix(original_input_path)
            .with_context(|| {
                format!(
                    "Internal Error: Gagal mendapatkan path relatif untuk {:?} dari {:?}",
                    current_file,
                    original_input_path
                )
            })?;
        let base_output_dir = original_input_path.join("elrem_output");
        Ok(base_output_dir.join(relative_path))
    } else {
        // Kasus Input File Tunggal:
        let parent_dir = current_file.parent().unwrap_or_else(|| Path::new("."));
        let stem = current_file
            .file_stem()
            .unwrap_or_default();
        let extension = current_file
            .extension()
            .unwrap_or_default();
        
        let new_filename = format!(
            "{}-nobg.{}", 
            stem.to_string_lossy(), 
            extension.to_string_lossy()
        );
        Ok(parent_dir.join(new_filename))
    }
} 