# ElRem - Penghapus Latar Belakang Gambar Sederhana

Selamat datang di ElRem! Aplikasi ini membantu Anda menghapus latar belakang (background) dari gambar secara otomatis melalui command-line (terminal).

## Fungsi Utama

*   Menghapus background dari satu file gambar (`.png`, `.jpg`, `.jpeg`).
*   Menghapus background dari semua gambar yang didukung dalam satu folder.
*   Menawarkan beberapa metode penghapusan background dengan kualitas dan kecepatan yang berbeda.

## Prasyarat (Yang Perlu Anda Siapkan)

Sebelum menggunakan ElRem, pastikan Anda sudah menginstall **rembg**.

1.  **Python 3:** Pastikan Python 3 terinstall di sistem Anda.
2.  **rembg:** Install `rembg` menggunakan pip (manajer paket Python):
    ```bash
    pip install rembg
    ```

*Catatan:* Metode penghapusan background yang lebih canggih (MODNet, SAM) memerlukan setup tambahan yang lebih teknis dan dijelaskan di dalam aplikasi jika Anda memilihnya.

## Cara Penggunaan

1.  **Buka Terminal:** Jalankan aplikasi ini dari terminal Anda.
2.  **Navigasi ke Direktori ElRem:** Pastikan Anda berada di direktori tempat Anda menyimpan atau meng-compile ElRem.
3.  **Jalankan Perintah:**

    *   **Untuk satu file gambar:**
        ```bash
        ./target/debug/elrem /path/ke/gambar/anda.jpg 
        ```
        (Ganti `./target/debug/elrem` dengan path ElRem jika berbeda, dan ganti `/path/ke/gambar/anda.jpg` dengan path file gambar Anda).

    *   **Untuk satu folder berisi gambar:**
        ```bash
        ./target/debug/elrem /path/ke/folder/anda
        ```
        (Ganti `./target/debug/elrem` dengan path ElRem jika berbeda, dan ganti `/path/ke/folder/anda` dengan path folder yang berisi gambar).

4.  **Pilih Metode:** Aplikasi akan menampilkan menu. Ketik nomor metode yang ingin Anda gunakan (misalnya `1` untuk metode tercepat) lalu tekan Enter.
5.  **Tunggu Proses:** Aplikasi akan memproses gambar Anda. Tunggu hingga selesai.

## Hasil Output

*   **Untuk input file tunggal:** Hasil gambar tanpa background akan disimpan di **folder yang sama** dengan file asli, dengan nama file yang ditambahkan akhiran `-nobg` (contoh: `gambar_anda-nobg.png`).
*   **Untuk input folder:** Hasil gambar akan disimpan di **subfolder baru bernama `elrem_output`** di dalam folder input Anda.

Selamat mencoba! 