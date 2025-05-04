// use colored::*;
use anyhow::{bail, Context, Result};
use dirs;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

// Gunakan path absolut crate untuk mengakses modul utils
use crate::utils;

/// Enum untuk merepresentasikan metode eksternal yang didukung.
#[derive(Debug, Clone, Copy)]
pub enum ExternalMethod {
    ModNet,
    Sam,
}

/// Memeriksa apakah setup untuk metode eksternal (MODNet/SAM) ada di `~/.elrem/`.
/// Mengembalikan `Ok(PathBuf)` ke skrip wrapper jika setup valid.
/// Mengembalikan `Err(String)` berisi instruksi setup jika tidak valid.
pub fn check_external_setup(method: ExternalMethod) -> Result<PathBuf, String> {
    let home_dir = dirs::home_dir().ok_or_else(|| "Kesalahan: Tidak dapat menemukan direktori home pengguna.".to_string())?;
    let base_elrem_path = home_dir.join(".elrem");

    match method {
        ExternalMethod::ModNet => {
            let method_path = base_elrem_path.join("MODNet");
            let script_path = method_path.join("run_modnet.py");
            let model_path = method_path.join("pretrained/modnet_photographic_portrait_matting.ckpt");

            if script_path.is_file() && model_path.is_file() {
                Ok(script_path)
            } else {
                let instructions = format!(
                    "Setup MODNet tidak ditemukan atau tidak lengkap di '{}'.\n\nLangkah Setup Manual:\n1. Clone repo MODNet:\n   git clone https://github.com/ZHKKKe/MODNet.git {}\n2. Unduh model pretrained 'modnet_photographic_portrait_matting.ckpt'\n   (Cari link di repo MODNet) dan letakkan di '{}/pretrained/'.\n3. Buat skrip wrapper Python `run_modnet.py` di '{}'\n   yang menerima `--input <file>` dan `--output <file>` (sesuaikan dari demo MODNet).\n4. Install dependensi Python:\n   cd {} && pip install -r requirements.txt",
                    method_path.display(),
                    method_path.display(),
                    method_path.display(),
                    method_path.display(),
                    method_path.display()
                );
                Err(instructions)
            }
        }
        ExternalMethod::Sam => {
            let method_path = base_elrem_path.join("segment-anything");
            let script_path = method_path.join("run_sam.py"); 
            let models_dir = method_path.join("models");

            if script_path.is_file() && models_dir.is_dir() {
                Ok(script_path)
            } else {
                 let instructions = format!(
                    "Setup SAM (Segment Anything) tidak ditemukan atau tidak lengkap di '{}'.\n\nLangkah Setup Manual:\n1. Clone repo SAM:\n   git clone https://github.com/facebookresearch/segment-anything.git {}\n2. Unduh model checkpoint SAM (misal, ViT-H *.pth)\n   (Link di repo SAM) dan letakkan di '{}/models/'.\n3. Buat skrip wrapper Python `run_sam.py` di '{}'\n   yang menerima `--input <file>` dan `--output <file>` (gunakan API SAM).\n4. Install dependensi Python:\n   cd {} && pip install -e . && pip install opencv-python pycocotools matplotlib onnxruntime onnx",
                    method_path.display(),
                    method_path.display(),
                    method_path.display(),
                    method_path.display(),
                    method_path.display()
                );
                Err(instructions)
            }
        }
    }
}

/// Menjalankan skrip Python eksternal (MODNet/SAM wrapper).
/// Memastikan Python ada, membuat direktori output, dan menangkap error.
pub fn run_external_script(script_path: &Path, input_path: &Path, output_path: &Path) -> Result<()> {
    if let Some(parent) = output_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).with_context(|| format!("Gagal membuat direktori output: {:?}", parent))?;
        }
    }

    // Panggil utils::check_command_exists
    let python_cmd = if utils::check_command_exists("python3")? {
        "python3"
    } else if utils::check_command_exists("python")? {
        "python"
    } else {
        bail!("Prasyarat Error: Tidak dapat menemukan perintah 'python3' atau 'python'.")
    };

    let output = Command::new(python_cmd)
        .arg(script_path)
        .arg("--input")
        .arg(input_path)
        .arg("--output")
        .arg(output_path)
        .output()
        .with_context(|| format!("Gagal mengeksekusi skrip eksternal: {}", script_path.display()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        bail!(
            "Eksekusi skrip '{}' gagal untuk '{}'. Kode: {}.\n--- Stderr: ---\n{}\n--- Stdout: ---\n{}",
            script_path.display(),
            input_path.display(),
            output.status,
            stderr.trim(),
            stdout.trim()
        );
    }
    Ok(())
} 
