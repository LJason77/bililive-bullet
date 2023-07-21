use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DanmuInfo {
    pub token: String,
    pub host_list: Vec<HostList>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HostList {
    pub host: String,
    pub port: u32,
    pub wss_port: u32,
    pub ws_port: u32,
}

impl DanmuInfo {
    pub async fn get_info(room_id: u64) -> Result<Self, serde_json::Error> {
        let mut body = reqwest::get(format!(
            "https://api.live.bilibili.com/xlive/web-room/v1/index/getDanmuInfo?id={room_id}"
        ))
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();
        serde_json::from_value::<Self>(body["data"].take())
    }
}
