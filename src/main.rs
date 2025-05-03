use anyhow::{bail, Context, Result};
use clap::Parser;
use colored::*;
// use dialoguer::{theme::ColorfulTheme, Select}; // Pindah ke ui
// use indicatif::{ProgressBar, ProgressStyle}; // Pindah ke processing
use std::fs;
use std::path::PathBuf;
// use walkdir::WalkDir; // Tidak perlu lagi di main
// use std::process::{Command, Stdio}; // Tidak perlu lagi di main (dipakai di utils, external, processing)

// Deklarasi modul baru
mod cli;
mod utils;
mod input; // Tambahkan deklarasi modul input
mod external; // Tambahkan deklarasi modul external
mod processing; // Tambahkan deklarasi modul processing
mod ui; // Tambahkan deklarasi modul ui

// --- Konstanta & Fungsi Helper (Sudah dipindah ke src/input/mod.rs) ---
// const ALLOWED_EXTENSIONS: [&str; 3] = ...;
// fn is_valid_image_extension(path: &Path) -> bool { ... }
// fn validate_single_file(path: &Path) -> Result<Vec<PathBuf>> { ... }
// fn collect_images_from_dir(dir_path: &Path) -> Result<Vec<PathBuf>> { ... }

// --- Fungsi yang sudah dipindah ke modul lain ---
// fn run_rembg(...) { ... } // Pindah ke processing
// enum ExternalMethod { ... } // Pindah ke external
// fn check_external_setup(...) -> Result<PathBuf, String> { ... } // Pindah ke external
// fn run_external_script(...) -> Result<()> { ... } // Pindah ke external
// fn process_images(...) -> Result<()> { ... } // Pindah ke processing

fn main() -> Result<()> {
    let args = cli::Args::parse();
    let expanded_input = shellexpand::tilde(&args.input_path);
    let input_path_buf = PathBuf::from(expanded_input.as_ref());

    // 1. Cek Prasyarat
    if !utils::check_command_exists("rembg")? {
        bail!(
            "{}",
            format!(
                "{}",
                "Error Prasyarat: Perintah 'rembg' tidak ditemukan.\nPastikan rembg terinstall (`pip install rembg`) dan ada di PATH sistem."
            ).red()
        );
    }
    if !(utils::check_command_exists("python3")? || utils::check_command_exists("python")?) {
         eprintln!("{}", "Warning: Perintah 'python3' atau 'python' tidak ditemukan. Metode eksternal (MODNet/SAM) tidak akan berfungsi.".yellow());
    }

    // 2. Validasi Input Path
     if !input_path_buf.exists() {
        bail!("Error Input: Path '{}' tidak ditemukan.", input_path_buf.display());
    }

    // 3. Identifikasi Tipe & Kumpulkan File Gambar (Gunakan modul input)
     let metadata = fs::metadata(&input_path_buf)
        .with_context(|| format!("Gagal membaca metadata untuk: {}", input_path_buf.display()))?;
    let input_files: Vec<PathBuf>;
    let is_dir_input = metadata.is_dir();
    let display_path = input_path_buf.display().to_string();

    if is_dir_input {
        println!("{} Input Direktori: Mencari gambar di '{}'...", "INFO:".blue(), display_path);
        // Panggil fungsi dari modul input
        input_files = crate::input::collect_images_from_dir(&input_path_buf)
            .with_context(|| format!("Gagal mengumpulkan gambar dari direktori: {}", display_path))?;
        println!("{} Pencarian selesai.", " OK:".green());
    } else if metadata.is_file() {
        println!("{} Input File: Memvalidasi '{}'...", "INFO:".blue(), display_path);
        // Panggil fungsi dari modul input
        input_files = crate::input::validate_single_file(&input_path_buf)
            .with_context(|| format!("Gagal memvalidasi file input: {}", display_path))?;
         println!("{} Validasi selesai.", " OK:".green()); // Tambah konfirmasi validasi
    } else {
        bail!(
            "Error Input: Path '{}' bukan file atau diretori yang valid.",
            display_path
        );
    }

    // 4. Cek Hasil Pengumpulan File
     if input_files.is_empty() {
        println!(
            "{}",
            "Info: Tidak ditemukan file gambar (.png, .jpg, .jpeg) yang valid untuk diproses.".yellow()
        );
        return Ok(());
    }
    println!(
        "{}",
        format!("{} Ditemukan {} file gambar yang akan diproses."," OK:".green(), input_files.len())
    );


    // 5. Tampilkan Menu Pilihan Metode (Gunakan modul ui)
    let selection = ui::select_method()?;

    // 6. Proses Pilihan Pengguna
     match selection {
        Some(index) => {
            // Indeks menu dari dialoguer adalah 0-based.
            // Menu kita: 0..=4 adalah metode, 5 adalah Exit.
            if index == 5 { // Index 5 (pilihan ke-6) adalah "Exit"
                println!("{}", "Keluar.".cyan());
                return Ok(());
            }
            // Panggil fungsi dari modul processing
            processing::process_images(input_files, index, is_dir_input, &input_path_buf)?;
        }
        None => {
            println!("{}", "Tidak ada pilihan dibuat, keluar.".yellow());
        }
    }


    Ok(())
} 