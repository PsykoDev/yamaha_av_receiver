use api;
use api::{http_get, System};
use detection::{ScanConfig, ScanType};
use std::error::Error;
use std::net::Ipv4Addr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // connection direct ip
    let ip = ScanConfig {
        ip: Ipv4Addr::new(192, 168, 1, 86),
    };
    let scan = ScanType::scan_or_direct(&ScanType::Direct(ip.clone())).await?;
    let data = &(scan.first()).unwrap().1;
    let object = ScanConfig::to_object(&data);
    println!("direct ip\nmodel_name: {}", object?["model_name"]);

    // test get data
    let test = http_get(&ip, System::GET_LOCATION_INFO).await?;
    let name_zone = ScanConfig::to_object(&test);
    let zone = ScanConfig::to_object(&test);
    println!(
        "direct ip\nAV name: {}\nZone main ?: {}",
        name_zone?["name"], zone?["zone_list"]["main"]
    );

    println!();

    // connection auto scan
    let auto_scan = ScanType::Auto.scan_or_direct().await?;
    let ip = &(auto_scan.first()).unwrap().0;
    let data = &(auto_scan.first()).unwrap().1;
    let name_zone = ScanConfig::to_object(&data);
    println!(
        "auto scan\nmodel_name: {}",
        name_zone?.clone()["model_name"]
    );

    //test get data
    let test = http_get(&ScanConfig { ip: *ip }, System::GET_LOCATION_INFO).await?;
    let name_zone = ScanConfig::to_object(&test);
    let zone = ScanConfig::to_object(&test);
    println!(
        "auto scan\nAV name: {}\nZone main ?: {}",
        name_zone?["name"], zone?["zone_list"]["main"]
    );

    Ok(())
}
