# Monitoring Suhu dan Kelembaban dalam Fermentasi Kakao untuk Menjaga Kualitas Cokelat


## 📘 Deskripsi Proyek
Proyek ini bertujuan untuk membangun sistem monitoring suhu dan kelembaban yang digunakan selama proses fermentasi biji kakao. Sistem menggunakan sensor industri SHT20 dengan komunikasi Modbus RTU, yang datanya dikirim melalui Modbus Client ke TCP Server berbasis bahasa Rust. Data yang diterima kemudian diproses dan disimpan dalam basis data time-series InfluxDB, serta divisualisasikan secara real-time menggunakan Grafana. Selain itu, sistem juga dilengkapi dengan aplikasi desktop berbasis Qt untuk pemantauan lokal. Dengan sistem ini, kondisi lingkungan fermentasi dapat dipantau secara akurat dan berkelanjutan, sehingga mendukung tercapainya kualitas biji kakao yang optimal.

---
## 🎓 Mata Kuliah
- **[Interkoneksi Sistem Instrumentasi]** –
  Program Studi Teknik Instrumentasi
  Dosen Pengampu: [Ahmad Radhy, S.Si., M.Si]

  ---
  ## 👨‍💻 Anggota Kelompok
| Nama | NIM | 
|------|-----|
| [Aulia Zakhrine Ramadhani Setiawan] | [2042231042] | 
| [Fadillah Wahyu Anggraini] | [2042231052] | 
| [Fortunia Putri Syahari] | [2042231078] | 

---
## 📚 State Of The Art
| Topik, Penulis, dan Tahun | Teknologi yang Digunakan | Hasil|
|------|-----|-----|
| Afoakwa, E. O. (2014) – Cocoa Production and Processing Technology|Pemantauan manual suhu dan kelembaban dalam kotak fermentasi tradisional, tanpa penyimpanan digital|Ditekankan pentingnya suhu dan kelembaban dalam menghasilkan kakao berkualitas tinggi, namun belum menggunakan teknologi monitoring otomatis|
|Schwan & Wheals (2004) – The Microbiology of Cocoa Fermentation|Studi mikrobiologi fermentasi kakao, tanpa integrasi sensor digital|Menjelaskan hubungan langsung antara suhu/kelembaban dengan aktivitas mikroba fermentasi; menyarankan kontrol parameter lingkungan untuk hasil fermentasi yang optimal|

---
## ⚙️ Fitur Utama
 ✅ Monitoring Suhu & Kelembaban Real-Time 
 ✅ Penyimpanan Data Historis 
 ✅ Visualisasi Data 
 ✅ Blockchain-based Data Logging, untuk Menjamin keaslian & keamanan data

---
## 🛠 Step by Step Eksekusi Sistem
1. Jalankan Program Sensor
 bash
cd tcp_servers
cargo build
cargo run
 Terminal baru
 bash
cd modbus_client
cargo build
cargo run

Sensor akan mengirim data suhu dan kelembaban secara berkala ke server melalui TCP.

2. Jalankan Ganache dan start_blockchain
Terminal Baru
cd sensor-blockchain
ganache

Terminal Baru
./start_blockchain.sh
lalu masukkan contract adress ke app.js 

Akan muncul 10 address dan private key untuk digunakan di Metamask.

3. Lalu klik Go live pada index.html maka akan muncul localhost 8545


let wallet: LocalWallet = "0x...".parse::<LocalWallet>()?

Lalu jalankan:
 bash
cargo run


✅ Output di terminal:

✅ Smart contract deployed at: 0xef9ec8f88f74cbac54b9e0065057de6c0b367570
🚪 TCP Server listening on port 9000...
📥 Received sensor data: ...
✅ InfluxDB: data written
📡 Ethereum: tx sent: ...


4. Edit File Web Dashboard
 bash
cd Web31


Buka file script.js dan ubah:
 bash
const contractAddress = "0xef9ec8f88f74cbac54b9e0065057de6c0b367570";


5. Jalankan Web Server
Akses browser ke: http://localhost:8545

6. Simulasi Transaksi Web3
- Buka Metamask, import private key dari Ganache
- Klik Load Sensor Data
- Masukkan nilai suhu dan kelembaban target
- Klik Simulate Purchase
- Sistem akan menampilkan QR code dan total ETH simulasi

## 📌 Saran untuk Pengembangan Selanjutnya
- Integrasi Kecerdasan Buatan (AI) untuk Prediksi dan Optimasi Penetasan 
- Fitur Prediksi Waktu Tetas Otomatis 
- Integrasi Kamera Mini untuk Pemantauan Visual 
- Fitur Marketplace Internal 

---
