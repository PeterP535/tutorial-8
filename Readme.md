# Dokumentasi Proyek gRPC Chat, Pembayaran, dan Transaksi dengan Rust

## Perbandingan Unary, Server Streaming, dan Bi-Directional Streaming RPC

**Unary RPC:** Klien mengirim satu permintaan dan menerima satu respons. Cocok untuk operasi sederhana seperti login, pembayaran, atau pengambilan data tunggal.

**Server Streaming RPC:** Klien mengirim satu permintaan, dan server mengirimkan serangkaian respons. Cocok untuk histori transaksi atau daftar panjang yang terus diperbarui.

**Bi-Directional Streaming RPC:** Klien dan server saling mengirimkan stream data secara independen. Sangat sesuai untuk aplikasi chat atau komunikasi real-time.

## Pertimbangan Keamanan dalam gRPC Rust

1. **Otentikasi:** Gunakan TLS/SSL dan token berbasis OAuth atau JWT untuk memverifikasi identitas klien/server.
2. **Otorisasi:** Validasi hak akses pengguna berdasarkan peran (role-based access control).
3. **Enkripsi:** Pastikan semua komunikasi antar pihak dienkripsi melalui TLS dan hindari penyimpanan data sensitif secara langsung tanpa perlindungan tambahan.

## Tantangan dalam Bi-Directional Streaming (Chat)

* **Sinkronisasi Stream:** Mengelola dua arah komunikasi secara paralel bisa kompleks, apalagi bila buffer penuh atau terjadi lag.
* **Error Handling:** Perlu strategi retry, timeouts, dan pemutusan koneksi aman.
* **Concurrency:** Butuh manajemen asinkron yang baik, terutama bila banyak klien terhubung secara bersamaan.

## Kelebihan dan Kekurangan `ReceiverStream` di Rust

**Kelebihan:**

* Integrasi langsung dengan kanal `mpsc` dari Tokio.
* Ringan dan cocok untuk implementasi server-stream secara reaktif.

**Kekurangan:**

* Buffer terbatas; pengiriman pesan yang terlalu cepat dapat membuat klien ketinggalan.
* Kurang fleksibel untuk alur yang sangat kompleks seperti logika berbasis waktu atau prioritas pesan.

## Struktur Kode Modular & Reusable

* **Pisahkan File:** Buat modul terpisah seperti `payment.rs`, `transaction.rs`, `chat.rs`, dan `server.rs`.
* **Gunakan Trait:** Gunakan trait untuk mendefinisikan interface dan implementasinya pada berbagai struct.
* **Konfigurasi:** Gunakan file `.env` atau `config.rs` untuk menyimpan endpoint dan variabel konfigurasi lainnya.

## Pengembangan Lanjutan pada `MyPaymentService`

* **Validasi Input:** Cek validitas `amount`, `user_id`, dan token otentikasi.
* **Integrasi Pihak Ketiga:** Hubungkan dengan sistem pembayaran nyata seperti Stripe, Midtrans, atau bank.
* **Manajemen Status Transaksi:** Simpan dan update status pembayaran di database dengan retry mechanism.

## Dampak gRPC terhadap Arsitektur Sistem Terdistribusi

* **Interoperabilitas Tinggi:** Kompatibel dengan banyak bahasa (C++, Python, Java, Go, dll).
* **Efisiensi:** Lebih cepat dari REST karena menggunakan HTTP/2 dan Protocol Buffers.
* **Kelemahan:** Butuh tooling dan setup lebih kompleks, serta kurang cocok untuk klien berbasis browser tanpa WebSocket.

## Perbandingan HTTP/2 (gRPC) dengan HTTP/1.1 dan WebSocket

**Keunggulan HTTP/2/gRPC:**

* Multiplexing: Beberapa permintaan dalam satu koneksi.
* Header Compression: Lebih hemat bandwidth.
* Built-in Streaming.

**Kekurangan:**

* Tidak semua browser mendukung gRPC penuh (butuh gRPC-web untuk frontend).

**WebSocket:**

* Cocok untuk komunikasi real-time browser, tapi lebih manual dan tidak seketat gRPC soal struktur data.

## Kontras REST vs gRPC (Real-time & Responsiveness)

REST menggunakan pola request-response satu arah. Tidak cocok untuk komunikasi real-time (butuh polling atau WebSocket tambahan).

gRPC mendukung streaming dua arah, menjadikannya unggul dalam:

* Respons instan
* Efisiensi bandwidth
* Responsivitas sistem secara keseluruhan

## Skema vs Tanpa Skema: gRPC vs REST (Protobuf vs JSON)

**gRPC (Protobuf):**

* Skema eksplisit → Validasi kuat, efisiensi tinggi.
* Tidak fleksibel terhadap perubahan skema mendadak.

**REST (JSON):**

* Skema bebas → Mudah untuk iterasi cepat.
* Rentan kesalahan parsing dan inkonsistensi antar versi API.




