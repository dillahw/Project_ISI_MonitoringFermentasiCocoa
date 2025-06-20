# Monitoring Suhu dan Kelembaban dalam Fermentasi Kakao untuk Menjaga Kualitas Cokelat


## 📘 Deskripsi Proyek
Proyek ini bertujuan untuk membangun sistem monitoring suhu dan kelembaban yang digunakan selama proses fermentasi biji kakao. Sistem menggunakan sensor industri SHT20 dengan komunikasi Modbus RTU, yang datanya dikirim melalui Modbus Client ke TCP Server berbasis bahasa Rust. Data yang diterima kemudian diproses dan disimpan dalam basis data time-series InfluxDB, serta divisualisasikan secara real-time menggunakan Grafana. Selain itu, sistem juga dilengkapi dengan aplikasi desktop berbasis Qt untuk pemantauan lokal. Dengan sistem ini, kondisi lingkungan fermentasi dapat dipantau secara akurat dan berkelanjutan, sehingga mendukung tercapainya kualitas biji kakao yang optimal.

---
## 🎓 Mata Kuliah
- **[Interkoneksi Sistem Instrumentasi]** – Program Studi Teknik Instrumentasi
- Dosen Pengampu: [Ahmad Radhy, S.Si., M.Si]

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
| Afoakwa, E. O. (2014) – Cocoa Production and Processing Technology|Pemantauan manual suhu dan kelembaban dalam kotak fermentasi tradisional, tanpa penyimpanan digital|Ditekankan pentingnya suhu dan kelembaban dalam menghasilkan kakao berkualitas tinggi, namun belum menggunakan teknologi monitoring otomatis|
|Schwan & Wheals (2004) – The Microbiology of Cocoa Fermentation|Studi mikrobiologi fermentasi kakao, tanpa integrasi sensor digital|Menjelaskan hubungan langsung antara suhu/kelembaban dengan aktivitas mikroba fermentasi; menyarankan kontrol parameter lingkungan untuk hasil fermentasi yang optimal|

---
## ⚙️ Fitur Utama
- ✅ Monitoring Suhu & Kelembaban Real-Time 
- ✅ Penyimpanan Data Historis 
- ✅ Visualisasi Data 
- ✅ Blockchain-based Data Logging, untuk Menjamin keaslian & keamanan data

  ## 🛠️ Implementasi dan Kode Program
**- Rust Modbust Client**
