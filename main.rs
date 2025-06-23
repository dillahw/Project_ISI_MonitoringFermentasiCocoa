use influxdb2::{Client as InfluxClient, models::DataPoint};
use serde::Deserialize;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener};
use futures::stream;
use chrono::{Utc, DateTime};
use dotenvy::dotenv;
use std::{env, sync::Arc};

use ethers::prelude::*;
use ethers::types::Address;

#[derive(Debug, Deserialize)]
struct SensorData {
    timestamp: String,
    sensor_id: String,
    location: String,
    process_stage: String,
    temperature_celsius: f64,
    humidity_percent: f64,
}

// Buat binding untuk smart contract
abigen!(
    SHT20Contract,
    r#"[
        function logData(int256 temperature, uint256 humidity) public
    ]"#
);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // InfluxDB setup
    let influx_url = "http://localhost:8086";
    let influx_org = "kopi";
    let influx_token = env::var("INFLUX_TOKEN")?;
    let influx_bucket = "SHT20";

    let influx_client = InfluxClient::new(influx_url, influx_org, influx_token);

    // Blockchain setup
    let eth_rpc_url = env::var("ETH_RPC_URL")?;
    let private_key = env::var("PRIVATE_KEY")?;
    let contract_address: Address = env::var("CONTRACT_ADDRESS")?.parse()?;

    let wallet: LocalWallet = private_key.parse::<LocalWallet>()?.with_chain_id(1337u64);
    let provider = Provider::<Http>::try_from(eth_rpc_url.clone())?.interval(std::time::Duration::from_secs(1));
    let signer = Arc::new(SignerMiddleware::new(provider, wallet));
    let contract = Arc::new(SHT20Contract::new(contract_address, signer));

    // TCP server
    let listener = TcpListener::bind("127.0.0.1:7878").await?;
    println!("Listening on 127.0.0.1:7878");

    loop {
        let (mut socket, _) = listener.accept().await?;
        let influx_client = influx_client.clone();
        let contract = contract.clone();
        let bucket = influx_bucket.to_string();

        tokio::spawn(async move {
            let mut buf = [0u8; 1024];
            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(0) => break,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("Read error: {}", e);
                        break;
                    }
                };

                let raw_data = match std::str::from_utf8(&buf[..n]) {
                    Ok(d) => d,
                    Err(e) => {
                        let _ = socket.write_all(b"ERROR: Invalid UTF-8").await;
                        eprintln!("UTF-8 error: {}", e);
                        continue;
                    }
                };

                match serde_json::from_str::<SensorData>(raw_data) {
                    Ok(sensor_data) => {
                        let timestamp = match DateTime::parse_from_rfc3339(&sensor_data.timestamp) {
                            Ok(dt) => dt.with_timezone(&Utc),
                            Err(e) => {
                                let _ = socket.write_all(b"ERROR: Bad timestamp").await;
                                eprintln!("Timestamp error: {}", e);
                                continue;
                            }
                        };

                        // Simpan ke InfluxDB
                        let point = DataPoint::builder("environment_monitoring")
                            .tag("sensor_id", &sensor_data.sensor_id)
                            .tag("location", &sensor_data.location)
                            .tag("process_stage", &sensor_data.process_stage)
                            .field("temperature_celsius", sensor_data.temperature_celsius)
                            .field("humidity_percent", sensor_data.humidity_percent)
                            .timestamp(timestamp.timestamp_nanos_opt().unwrap_or(0))
                            .build().unwrap();

                        match influx_client.write(&bucket, stream::iter(vec![point])).await {
                            Ok(_) => println!("InfluxDB: OK"),
                            Err(e) => eprintln!("Influx write error: {}", e),
                        }

                        // Kirim ke Blockchain
                        let temp_i32 = (sensor_data.temperature_celsius * 100.0) as i32;
                        let humid_u32 = (sensor_data.humidity_percent * 100.0) as u32;

                        match contract.log_data(temp_i32.into(), humid_u32.into()).send().await {
                            Ok(pending_tx) => {
                                println!("Blockchain tx hash: {:?}", pending_tx.tx_hash());
                                let _ = socket.write_all(b"OK").await;
                            }
                            Err(e) => {
                                eprintln!("Blockchain error: {}", e);
                                let _ = socket.write_all(b"ERROR: Blockchain").await;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("JSON error: {}", e);
                        let _ = socket.write_all(b"ERROR: JSON").await;
                    }
                }
            }
        });
    }
}
