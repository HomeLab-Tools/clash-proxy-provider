use std::collections::BTreeMap as Map;
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
    #[serde(default)]
    plugin: String,
    #[serde(default, rename="plugin-opts")]
    plugin_obfs: Map<String, String>

}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub proxies: serde_yaml::Value,
}
