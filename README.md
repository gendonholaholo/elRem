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

| Gambar Asli | Hasil Tanpa Background |
|-------------|------------------------|
| ![image](https://github.com/user-attachments/assets/78217015-9052-44e9-95b3-b396986b3dbe) | ![image-nobg](https://github.com/user-attachments/assets/6aaa812a-3c93-4e4c-badb-5f94a9ef47c5) |

| elRem |
|----------------------|
| ![1](https://github.com/user-attachments/assets/c6fde819-a85a-44be-b377-3cfec15ecb14) |
| ![3](https://github.com/user-attachments/assets/e2adc9a9-4a2f-427b-a2c4-181ec690c02e) |

*   **Untuk input file tunggal:** Hasil gambar tanpa background akan disimpan di **folder yang sama** dengan file asli, dengan nama file yang ditambahkan akhiran `-nobg` (contoh: `gambar_anda-nobg.png`).
*   **Untuk input folder:** Hasil gambar akan disimpan di **subfolder baru bernama `elrem_output`** di dalam folder input Anda.
Selamat mencoba!
