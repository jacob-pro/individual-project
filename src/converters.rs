use crate::Common;
use crate::config::{ConfigDisk, ConfigMachine};
use crate::virt_util::devices::{DiskDriverType, DiskDevice, DiskXml, TargetBus, DeviceXML};
use crate::virt_util::domain::DomainXml;
use crate::virt_util::devices::GraphicsXml;

impl ConfigDisk {

    pub fn to_virt_disk(&self, common: &Common) -> anyhow::Result<DiskXml> {
        Ok(match self {
            ConfigDisk::CloudImage { cloud_image } => {
                let image_path = cloud_image.resolve_path(&common)?.canonicalize()?;
                DiskXml::new(
                    DiskDriverType::QCow2,
                    image_path.to_str().unwrap().to_owned(),
                    DiskDevice::Disk,
                    true,
                    "hdc".to_string(),
                    TargetBus::VirtIO,
                )
            }
            ConfigDisk::Path { path } => {
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
            .memory(self.memory)
            .cpus(self.cpus)
            .device(DeviceXML::Graphics(vnc));
        for disk in &self.disks {
            let disk = disk.to_virt_disk(&common)?;
            builder = builder.device(DeviceXML::Disk(disk));
        }
        Ok(builder.build().unwrap())
    }

}
