use std::error::Error;

use detection::ScanConfig;

pub struct System(&'static str);

impl System {
    pub const GET_DEVICE_INFO: Self = Self("/v1/system/getDeviceInfo");
    pub const GET_FEATURES: Self = Self("/v1/system/getFeatures");
    pub const GET_FUNC_STATUS: Self = Self("/v1/system/getFuncStatus");
    pub const GET_LOCATION_INFO: Self = Self("/v1/system/getLocationInfo");
}

pub struct Zone(&'static str);

impl Zone {
    pub const SET_POWER: Self = Self("/v1/main/setPower?power=toggle");
}

pub trait RequestTrait {
    fn get_path(&self) -> &'static str;
}

impl RequestTrait for System {
    fn get_path(&self) -> &'static str {
        self.0
    }
}

impl RequestTrait for Zone {
    fn get_path(&self) -> &'static str {
        self.0
    }
}

pub async fn http_get<T: RequestTrait>(
    ip: &ScanConfig,
    request: T,
) -> Result<String, Box<dyn Error>> {
    let body = reqwest::get(format!(
        "http://{}/YamahaExtendedControl{}",
        ip.ip,
        request.get_path()
    ))
    .await?;
    Ok(body.text().await?)
}

pub fn response_code_parser(id: u8) -> &'static str {
    match id {
        0 => "0 Successful request",
        1 => "1 Initializing",
        2 => "2 Internal Error",
        3 => "3 Invalid Request (A method did not exist, a method wasn’t appropriate etc.)",
        4 => "4 Invalid Parameter (Out of range, invalid characters etc.)",
        5 => "5 Guarded (Unable to setup in current status etc.)",
        6 => "6 Time Out",
        99 => "99 Firmware Updating",
        100 => "(Streaming Service related errors) \n\t100 Access Error",
        101 => "(Streaming Service related errors) \n\t101 Other Errors",
        102 => "(Streaming Service related errors) \n\t102 Wrong User Name",
        103 => "(Streaming Service related errors) \n\t103 Wrong Password",
        104 => "(Streaming Service related errors) \n\t104 Account Expired",
        105 => "(Streaming Service related errors) \n\t105 Account Disconnected/Gone Off/Shut Down",
        106 => "(Streaming Service related errors) \n\t106 Account Number Reached to the Limit",
        107 => "(Streaming Service related errors) \n\t107 Server Maintenance",
        108 => "(Streaming Service related errors) \n\t108 Invalid Account",
        109 => "(Streaming Service related errors) \n\t109 License Error",
        110 => "(Streaming Service related errors) \n\t110 Read Only Mode",
        111 => "(Streaming Service related errors) \n\t111 Max Stations",
        112 => "(Streaming Service related errors) \n\t112 Access Denied",
        113 => "(Streaming Service related errors) \n\t113 There is a need to specify the additional destination Playlist",
        114 => "(Streaming Service related errors) \n\t114 There is a need to create a new Playlist",
        115 => "(Streaming Service related errors) \n\t115 Simultaneous logins has reached the upper limit",
        200 => "(distribution related errors) \n\t200 Linking in progress",
        201 => "(distribution related errors) \n\t201 Unlinking in progress",
        _ => "No Error Found",
    }
}
