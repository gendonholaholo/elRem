use clap::Parser;

/// CLI untuk menghapus background gambar menggunakan berbagai metode.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path ke file gambar atau direktori berisi gambar.
    #[arg(required = true)]
    pub input_path: String, // Jadikan field pub agar bisa diakses
} 