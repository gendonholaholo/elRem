use anyhow::{bail, Context, Result};
use colored::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

// Impor modul lain dalam crate
use crate::external;
use crate::ui;
use crate::utils;

/// Menjalankan perintah rembg pada satu file.
/// (Private function, hanya digunakan dalam modul ini)
fn run_rembg(model: &str, input_path: &Path, output_path: &Path) -> Result<()> {
    if let Some(parent) = output_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Gagal membuat direktori output: {:?}", parent))?;
        }
    }
    let output = Command::new("rembg")
        .arg("i")
        .arg("-m")
        .arg(model)
        .arg(input_path)
        .arg(output_path)
        .output()
        .with_context(|| {
            format!(
                "Gagal mengeksekusi perintah rembg pada: {}",
                input_path.display()
            )
        })?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!(
            "Perintah rembg gagal untuk '{}'. Kode: {}. Stderr:\n{}",
            input_path.display(),
            output.status,
            stderr.trim()
        );
    }
    Ok(())
}

/// Fungsi utama pemrosesan: memilih metode, memeriksa setup jika perlu, dan memproses file.
pub fn process_images(
    files: Vec<PathBuf>,
    selected_method_index: usize,
    is_dir_input: bool,
    original_input_path: &Path,
) -> Result<()> {
    let method_info = match selected_method_index {
        0 => (true, "u2netp".to_string(), None),
        1 => (true, "u2net".to_string(), None),
        2 => (true, "isnet-general-use".to_string(), None),
        3 => (
            false,
            "ModNet".to_string(),
            Some(external::ExternalMethod::ModNet),
        ), // Gunakan path modul
        4 => (
            false,
            "Sam".to_string(),
            Some(external::ExternalMethod::Sam),
        ), // Gunakan path modul
        _ => unreachable!("Index menu tidak valid - ini seharusnya tidak terjadi"),
    };
    let (use_rembg, method_name, external_method_type) = method_info;

    let mut script_path_resolved: Option<PathBuf> = None;

    // Pemeriksaan setup khusus untuk metode eksternal
    if let Some(ext_method) = external_method_type {
        println!(
            "{} Memeriksa setup untuk metode eksternal: {}",
            "INFO:".blue(),
            method_name.yellow()
        );
        // Gunakan external::check_external_setup
        match external::check_external_setup(ext_method) {
            Ok(path) => {
                println!(
                    "{} Setup ditemukan: {}",
                    " OK:".green(),
                    path.display().to_string().cyan()
                );
                script_path_resolved = Some(path);
            }
            Err(instruction) => {
                eprintln!("\n{}", "SETUP DIPERLUKAN".red().bold());
                eprintln!("--------------------------------------------");
                eprintln!("{}", instruction);
                eprintln!("--------------------------------------------");
                eprintln!(
                    "\n{} Silakan ikuti langkah-langkah setup manual di atas.",
                    "INFO:".blue()
                );
                return Ok(());
            }
        }
    } else {
        println!(
            "{} Menggunakan model rembg: {}",
            "INFO:".blue(),
            method_name.cyan()
        );
    }

    // Buat Progress Bar menggunakan modul ui
    let pb = ui::create_progress_bar(files.len() as u64)?;

    let mut error_count = 0;
    let mut error_details = Vec::new();

    println!(
        "{} Memulai pemrosesan {} file...",
        "INFO:".blue(),
        files.len()
    );
    for file in files.iter() {
        let file_display = file
            .strip_prefix(original_input_path)
            .unwrap_or(file)
            .display()
            .to_string();
        pb.set_message(file_display.clone());
        pb.tick();

        // Gunakan utils::determine_output_path
        let result = utils::determine_output_path(file, is_dir_input, original_input_path)
            .and_then(|output_path| {
                if use_rembg {
                    // Panggil run_rembg lokal
                    run_rembg(&method_name, file, &output_path)
                } else {
                    let script = script_path_resolved.as_ref().expect(
                        "Logic error: script_path harus Some jika external method & setup OK",
                    );
                    // Gunakan external::run_external_script
                    external::run_external_script(script, file, &output_path)
                }
            });

        if let Err(e) = result {
            error_count += 1;
            error_details.push(format!("  - File '{}':\n    {:#}", file_display, e));
        }

        pb.inc(1);
    }

    pb.finish_and_clear();

    // Tampilkan laporan hasil akhir
    let success_count = files.len() - error_count;
    if error_count == 0 {
        println!(
            "{}",
            format!(
                "SUKSES! Semua {} file berhasil diproses dengan metode {}.",
                success_count, method_name
            )
            .green()
            .bold()
        );
    } else {
        println!(
            "{}",
            format!(
                "SELESAI dengan {} error. {}/{} file berhasil diproses dengan metode {}.",
                error_count,
                success_count,
                files.len(),
                method_name
            )
            .yellow()
        );
        eprintln!("\n{}", "DETAIL ERROR:".red().bold());
        eprintln!("--------------------------------------------");
        for detail in error_details {
            eprintln!("{}", detail);
            eprintln!("---");
        }
        eprintln!("--------------------------------------------");
    }

    Ok(())
}

