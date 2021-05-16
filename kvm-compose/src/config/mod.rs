pub mod convert;
pub mod images;
mod nocloud;

use crate::config::images::OnlineCloudImage;
use crate::virt_util::{DiskDeviceType, DiskDriverType};
use crate::Common;
use anyhow::Context;
use serde::Deserialize;
use std::borrow::Cow;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use validator::{Validate, ValidationError};

#[derive(Deserialize, Debug)]
pub struct ConfigInterface {
    pub bridge: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ConfigDisk {
    CloudImage {
        name: OnlineCloudImage,
        expand_gigabytes: Option<u16>,
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
}

#[derive(Deserialize, Debug, Validate)]
pub struct ConfigMachine {
    pub name: String,
    #[serde(default)]
    pub interfaces: Vec<ConfigInterface>,
    pub memory_mb: Option<u32>,
    pub cpus: Option<u32>,
    pub disk: ConfigDisk,
    pub run_script: Option<PathBuf>,
    #[validate(custom = "validate_context")]
    pub context: Option<PathBuf>,
    pub environment: HashMap<String, String>,
}

fn validate_context(context: &PathBuf) -> Result<(), ValidationError> {
    if !context.exists() {
        return Err(ValidationError {
            code: Cow::from(format!("{:?} does not exist", context)),
            message: None,
            params: HashMap::new(),
        });
    }
    Ok(())
}

#[derive(Deserialize, Debug, Validate)]
pub struct ConfigBridge {
    pub name: String,
    #[serde(default)]
    pub connect_external_interfaces: Vec<String>,
    #[serde(default)]
    pub enable_dhcp_client: bool,
    pub controller: Option<String>,
    pub protocol: Option<String>,
}

#[derive(Deserialize, Debug, Validate)]
pub struct Config {
    #[validate]
    pub machines: Vec<ConfigMachine>,
    #[validate]
    pub bridges: Vec<ConfigBridge>,
    pub ssh_public_key: String,
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
