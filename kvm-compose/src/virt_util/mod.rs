pub mod devices;
pub mod domain;
pub mod xml_tools;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DiskDriverType {
    Raw,
    QCow2,
}

impl Default for DiskDriverType {
    fn default() -> Self {
        Self::Raw
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DiskDeviceType {
    Disk,
    CdRom,
}

impl Default for DiskDeviceType {
    fn default() -> Self {
        Self::Disk
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TargetBus {
    Ide,
    VirtIO,
}

impl Default for TargetBus {
    fn default() -> Self {
        Self::Ide
    }
}
