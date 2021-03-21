use crate::images::OnlineCloudImage;
use anyhow::Context;
use serde::Deserialize;
use std::net::Ipv4Addr;
use std::path::{Path, PathBuf};
use validator::Validate;

#[derive(Deserialize, Debug)]
pub struct ConfigInterface {
    pub ipv4_address: Ipv4Addr,
    pub ipv4_mask: Ipv4Addr,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ConfigDisk {
    CloudImage { cloud_image: OnlineCloudImage },
    Path { path: PathBuf },
}

#[derive(Deserialize, Debug, Validate)]
pub struct ConfigMachine {
    pub name: String,
    pub interfaces: Vec<ConfigInterface>,
    pub memory: Option<u32>,
    pub cpus: Option<u32>,
    pub disks: Vec<ConfigDisk>,
}

#[derive(Deserialize, Debug, Validate)]
pub struct Config {
    #[validate]
    pub machines: Vec<ConfigMachine>,
}

impl Config {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let text = std::fs::read_to_string(path).with_context(|| "Reading Config file")?;
        let value: Self = serde_yaml::from_str(&text).with_context(|| "Parsing Config YAML")?;
        value.validate()?;
        Ok(value)
    }
}
