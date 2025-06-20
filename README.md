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
-✅ Monitoring Suhu & Kelembaban Real-Time 
-✅ Penyimpanan Data Historis 
-✅ Visualisasi Data 
-✅ Blockchain-based Data Logging, untuk Menjamin keaslian & keamanan data

  ## 🛠️ Implementasi dan Kode Program
**-Rust Modbust Client**
vuhih
**-main.rs** 
Modbus_client
use chrono::{Local, SecondsFormat};  
use tokio_modbus::{client::rtu, prelude::*};
use tokio_serial::SerialStream;
use tokio::{
    net::TcpStream,
    time::{sleep, Duration},
    io::{AsyncReadExt, AsyncWriteExt},
};
use serde_json::json;
use std::error::Error;


async fn sht20(slave: u8) -> Result<Vec<u16>, Box<dyn Error>> {
    let port = tokio_serial::new("/dev/ttyUSB0", 9600)
        .parity(tokio_serial::Parity::None)
        .stop_bits(tokio_serial::StopBits::One)
        .data_bits(tokio_serial::DataBits::Eight)
        .timeout(Duration::from_secs(1));
   
    let port = SerialStream::open(&port)?;
    let slave = Slave(slave);
   
    let response = {
        let mut ctx = rtu::attach_slave(port, slave);
        ctx.read_input_registers(1, 2).await?
    };


    Ok(response)
}


async fn send_to_server(
    sensor_id: &str,
    location: &str,
    process_stage: &str,
    temperature: f32,
    humidity: f32,
    timestamp: chrono::DateTime<Local>,  
) -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:7878").await?;
   
    let payload = json!({
        "timestamp": timestamp.to_rfc3339_opts(SecondsFormat::Secs, true),
        "sensor_id": sensor_id,
        "location": location,
        "process_stage": process_stage,
        "temperature_celsius": temperature,  
        "humidity_percent": humidity        
    });


    let json_str = payload.to_string();
    println!("Sending JSON: {}", json_str);
   
    stream.write_all(json_str.as_bytes()).await?;
   
    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).await?;
    println!("Server response: {}", std::str::from_utf8(&buf[..n])?);
   
    Ok(())
}


**-Rust TCP Server**
#[tokio::main]

async fn main() -> Result<(), Box<dyn Error>> {
    let sensor_id = "SHT20-PascaPanen-001";
    let location = "Gudang Fermentasi 1";
    let process_stage = "Fermentasi";
   
    loop {
        let timestamp = Local::now();  // Ubah dari Utc::now() ke Local::now()
       
        match sht20(1).await {
            Ok(response) if response.len() == 2 => {
                let temp = response[0] as f32 / 10.0;
                let rh = response[1] as f32 / 10.0;
               
                println!("[{}] {} - {}: Temp={:.1}°C, RH={:.1}%",
                    timestamp.format("%Y-%m-%d %H:%M:%S"),
                    location,
                    process_stage,
                    temp,
                    rh);
               
                if let Err(e) = send_to_server(
                    sensor_id,
                    location,
                    process_stage,
                    temp,
                    rh,
                    timestamp  // Tetap menggunakan timestamp yang sama
                ).await {
                    eprintln!("Failed to send data: {}", e);
                }
            }
            Ok(invalid) => eprintln!("Invalid sensor response: {:?}", invalid),
            Err(e) => eprintln!("Sensor read error: {}", e),
        }
       
        sleep(Duration::from_secs(10)).await;
    }
}

**-Cargo.homl**

cargo.homl Modbus_client
[package]
name = "sht20"
version = "0.1.0"
edition = "2021"


[dependencies]
chrono = "0.4"
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
tokio-modbus = "0.9"
tokio-serial = "5.4"

---

## 📚 Pengujian 
**- Hasil Pembacaan Suhu dan Kelembapan selama proses Inkubasi**
**- Hasil penyimpanan di InfluxDB.**
**- Visualisasi data Real-time dashboard di Grafana.**


---

## 📚 Analisa Hasil Pengujian
 
---

## 🖼️ Dokumentasi dan Demo

📹 Tonton video demo: 

📷 Lihat dokumentasi foto di folder


---

## 📌 Saran untuk Pengembangan Selanjutnya
- Integrasi Kecerdasan Buatan (AI) untuk Prediksi dan Optimasi Penetasan 
- Fitur Prediksi Waktu Tetas Otomatis 
- Integrasi Kamera Mini untuk Pemantauan Visual 
- Fitur Marketplace Internal 

---
