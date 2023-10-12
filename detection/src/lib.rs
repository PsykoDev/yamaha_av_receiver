extern crate core;

use core::fmt;
use std::error::Error;
use std::fmt::{Debug, Formatter};
use std::net::Ipv4Addr;
use std::sync::Arc;
use std::time::Duration;

use reqwest::{Client, StatusCode};
use serde_json::Value;

pub struct ScanConfig {
    pub ip: Ipv4Addr,
}

impl ScanConfig {
    async fn auto_scan() -> Result<Vec<(Ipv4Addr, String)>, Box<dyn Error>> {
        let client = Client::builder().timeout(Duration::new(1, 0)).build()?;
        let client = Arc::new(client);
        rayon::ThreadPoolBuilder::new()
            .num_threads(2)
            .build_global()?;

        let tasks: Vec<_> = (2..=254)
            .map(|i| Ipv4Addr::new(192, 168, 1, i))
            .map(|ip| {
                let url = format!(
                    "http://{}/YamahaExtendedControl/v1/system/getDeviceInfo",
                    ip
                );
                let client = client.clone();

                async move {
                    match client.get(&url).send().await {
                        Ok(response) if response.status() == StatusCode::OK => {
                            println!("IP found {}", ip);
                            let content = response.text().await.unwrap();
                            Some((ip, content))
                        }
                        Err(_) => None,
                        _ => None,
                    }
                }
            })
            .collect();

        let x = futures::future::join_all(tasks).await;
        let content_results: Vec<(Ipv4Addr, String)> =
            x.into_iter().filter_map(|result| result).collect();
        Ok(content_results)
    }

    async fn direct_connect(ip: &ScanConfig) -> Result<Vec<(Ipv4Addr, String)>, Box<dyn Error>> {
        let target_ip = format!(
            "http://{}/YamahaExtendedControl/v1/system/getDeviceInfo",
            ip.ip
        );
        let body = reqwest::get(&target_ip).await?;

        if body.status() == StatusCode::OK {
            Ok(vec![(ip.ip, body.text().await?)])
        } else {
            Ok(vec![])
        }
    }

    pub fn to_object(object: &str) -> Result<Value, Box<dyn Error>> {
        let v: Value = serde_json::from_str(object)?;
        Ok(v)
    }
}

impl Clone for ScanConfig {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for ScanConfig {}

impl Debug for ScanConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.ip)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ScanType {
    Direct(ScanConfig),
    Auto,
}

impl ScanType {
    pub async fn scan_or_direct(&self) -> Result<Vec<(Ipv4Addr, String)>, Box<dyn Error>> {
        match self {
            ScanType::Auto => ScanConfig::auto_scan().await,
            ScanType::Direct(e) => ScanConfig::direct_connect(e).await,
        }
    }
}
