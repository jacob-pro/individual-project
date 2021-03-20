use serde::Deserialize;
use std::path::{Path, PathBuf};
use anyhow::Context;
use std::net::Ipv4Addr;
use validator::{Validate};
use crate::images::OnlineCloudImage;
use crate::Common;

#[derive(Deserialize, Debug)]
pub struct Interface {
    pub ipv4_address: Ipv4Addr,
    pub ipv4_mask: Ipv4Addr,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Image {
    Named{name: OnlineCloudImage },
    Path{path: PathBuf},
}

#[derive(Deserialize, Debug, Validate)]
pub struct Machine {
    pub name: String,
    pub interfaces: Vec<Interface>,
    pub memory: Option<u32>,
    pub cpus: Option<u32>,
    image: Image,
}

impl Machine {

    pub fn get_image_path(&self, common: &Common) -> anyhow::Result<PathBuf> {
        return Ok(match &self.image {
            Image::Named { name } => {
                name.resolve_path(common)?
            }
            Image::Path { path } => { path.clone() }
        })
    }
}

#[derive(Deserialize, Debug, Validate)]
pub struct Config {
    #[validate]
    pub machines: Vec<Machine>,
}

impl Config {

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let text = std::fs::read_to_string(path).with_context(|| "Reading Config file")?;
        let value: Self = serde_yaml::from_str(&text).with_context(|| "Parsing Config YAML")?;
        value.validate()?;
        Ok(value)
    }

}
