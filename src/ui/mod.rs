use anyhow::Result;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Select};
use indicatif::{ProgressBar, ProgressStyle};

/// Menampilkan menu interaktif untuk memilih metode penghapusan latar belakang.
pub fn select_method() -> Result<Option<usize>> {
    let items = vec![
        "1. rembg - u2netp (Cepat, Kualitas Sedang)",
        "2. rembg - u2net (Normal, Kualitas Baik)",
        "3. rembg - isnet-general-use (Lambat, Kualitas Tinggi)",
        "4. ModNet (Perlu Setup Manual)",
        "5. SAM (Perlu Setup Manual)",
        "6. Exit",
    ];

    println!("{}", "Pilih metode penghapusan latar belakang:".green());
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_opt()?; // Menggunakan interact_opt untuk menangani pembatalan (ESC)

    Ok(selection)
}

/// Membuat dan mengkonfigurasi instance ProgressBar.
pub fn create_progress_bar(total_files: u64) -> Result<ProgressBar> {
    let pb = ProgressBar::new(total_files);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({percent}%) | ETA: {eta} | {msg:.dim}")?
            .progress_chars("━▶ "),
    );
    pb.set_message("Mempersiapkan...");
    Ok(pb)
}

// Fungsi lain yang mungkin dibutuhkan untuk UI, seperti konfirmasi:
// pub fn get_confirmation(prompt: &str) -> Result<bool> {
//     // Implementasi konfirmasi jika diperlukan
//     unimplemented!()
// }

