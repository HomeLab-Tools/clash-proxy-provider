use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Proxy {
    pub name: String,
    #[serde(rename = "type")]
    typ: String,
    pub server: String,
    port: u16,
    #[serde(default)]
    cipher: String,
    #[serde(default)]
    password: String,
    #[serde(default)]
    udp: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub proxies: serde_yaml::Value,
}
