use serde::Deserialize;
use std::path::Path;
use anyhow::Context;
use std::net::Ipv4Addr;

#[derive(Deserialize, Debug, Clone)]
pub struct Interface {
    pub ipv4_address: Ipv4Addr,
    pub ipv4_mask: Ipv4Addr,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Machine {
    pub name: String,
    pub interfaces: Vec<Interface>,
    pub memory: Option<u32>,
    pub cpus: Option<u32>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub machines: Vec<Machine>,
}

impl Config {

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let text = std::fs::read_to_string(path).with_context(|| "Reading Config file")?;
        let val: Self = serde_yaml::from_str(&text).with_context(|| "Parsing Config YAML")?;
        Ok(val)
    }

}
