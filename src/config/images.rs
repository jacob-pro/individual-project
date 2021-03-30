use crate::config::nocloud::CloudInitType;
use crate::download::download_file;
use crate::Common;
use enum_iterator::IntoEnumIterator;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, IntoEnumIterator)]
#[serde(rename_all = "lowercase")]
#[allow(non_camel_case_types)]
pub enum OnlineCloudImage {
    Ubuntu_18_04,
    Cirros_0_5_1,
}

impl OnlineCloudImage {
    fn get_url(&self) -> &str {
        match &self {
            OnlineCloudImage::Ubuntu_18_04 => {
                "https://cloud-images.ubuntu.com/bionic/current/bionic-server-cloudimg-amd64.img"
            }
            OnlineCloudImage::Cirros_0_5_1 => {
                "http://download.cirros-cloud.net/0.5.1/cirros-0.5.1-x86_64-disk.img"
            }
        }
    }

    pub fn resolve_path(&self, common: &Common) -> anyhow::Result<PathBuf> {
        let mut name = Common::storage_location()?;
        name.push(format!("{}.img", serde_plain::to_string(self).unwrap()));
        if !name.is_file() {
            common.confirm_continue(format!("Download image {:?} to {:?}?", self, name).as_str());
            download_file(self.get_url(), &name, 0o444)?;
        }
        Ok(name)
    }

    pub fn print_image_list() {
        println!("Available cloud images:");
        for i in Self::into_enum_iter() {
            println!("{}", serde_plain::to_string(&i).unwrap());
        }
    }

    pub fn get_cloud_init_type(&self) -> CloudInitType {
        match &self {
            OnlineCloudImage::Cirros_0_5_1 => CloudInitType::CirrosInit,
            _ => CloudInitType::CloudInit,
        }
    }
}
