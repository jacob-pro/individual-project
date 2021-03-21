pub mod convert;
pub mod images;
mod nocloud;

use crate::config::images::OnlineCloudImage;
use crate::virt_util::{DiskDeviceType, DiskDriverType};
use crate::Common;
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
#[serde(rename_all = "snake_case")]
pub enum ConfigDisk {
    CloudImage {
        name: OnlineCloudImage,
    },
    ExistingDisk {
        path: PathBuf,
        #[serde(default)]
        driver_type: DiskDriverType,
        #[serde(default)]
        device_type: DiskDeviceType,
        #[serde(default)]
        readonly: bool,
    },
    NewDisk {
        size_gb: u32,
    },
}

#[derive(Deserialize, Debug, Validate)]
pub struct CloudInit {
    pub meta_data: PathBuf,
    pub user_data: PathBuf,
}

#[derive(Deserialize, Debug, Validate)]
pub struct ConfigMachine {
    pub name: String,
    pub interfaces: Vec<ConfigInterface>,
    pub memory_mb: Option<u32>,
    pub cpus: Option<u32>,
    pub disks: Vec<ConfigDisk>,
    pub cloud_init: Option<CloudInit>,
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

impl ConfigMachine {
    pub fn get_virt_name(&self, common: &Common) -> String {
        format!("{}-{}", common.project, self.name)
    }
}
