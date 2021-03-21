use crate::Common;
use crate::config::{ConfigDisk, ConfigMachine};
use crate::virt_util::devices::{DiskXml, DeviceXML};
use crate::virt_util::domain::DomainXml;
use crate::virt_util::devices::GraphicsXml;
use crate::virt_util::{DiskDriverType, DiskDeviceType, TargetBus};

impl ConfigDisk {

    pub fn to_virt_disk(&self, common: &Common) -> anyhow::Result<DiskXml> {
        Ok(match self {
            ConfigDisk::CloudImage { name } => {
                let image_path = name.resolve_path(&common)?.canonicalize()?;
                DiskXml::new(
                    DiskDriverType::QCow2,
                    image_path.to_str().unwrap().to_owned(),
                    DiskDeviceType::Disk,
                    true,
                    "hdc".to_string(),
                    TargetBus::VirtIO,
                )
            }
            ConfigDisk::ExistingDisk { path, driver_type, device_type, readonly } => {
                DiskXml::new(
                    driver_type.clone(),
                    path.canonicalize()?.to_str().unwrap().to_owned(),
                    device_type.clone(),
                    *readonly,
                    "hda".to_string(),
                    TargetBus::Ide,
                )
            }
            ConfigDisk::NewDisk { .. } => {
                unimplemented!()
            }
        })
    }

}

impl ConfigMachine {

    pub fn virt_name(&self, common: &Common) -> String {
        format!("{}-{}", common.project, self.name)
    }

    pub fn to_virt_domain(&self, common: &Common) -> anyhow::Result<DomainXml> {
        let vnc = GraphicsXml::new("vnc".to_owned(), "-1".to_owned(), "yes".to_owned());
        let mut builder = DomainXml::builder()
            .name(&self.virt_name(&common))
            .memory(self.memory_mb)
            .cpus(self.cpus)
            .device(DeviceXML::Graphics(vnc));
        for disk in &self.disks {
            let disk = disk.to_virt_disk(&common)?;
            builder = builder.device(DeviceXML::Disk(disk));
        }
        Ok(builder.build().unwrap())
    }

}
