use crate::config::nocloud::{genisoimage, MetaData};
use crate::config::{CloudInit, ConfigDisk, ConfigMachine};
use crate::virt_util::devices::GraphicsXml;
use crate::virt_util::devices::{DeviceXML, DiskXml};
use crate::virt_util::domain::DomainXml;
use crate::virt_util::{DiskDeviceType, DiskDriverType, TargetBus};
use crate::Common;
use anyhow::Context;
use std::path::PathBuf;

#[derive(new)]
pub struct MachineToDomainConverter<'t> {
    common: &'t Common,
    machine: &'t ConfigMachine,
}

impl<'t> MachineToDomainConverter<'t> {
    fn create_virt_disk(&self, disk: &ConfigDisk) -> anyhow::Result<DiskXml> {
        Ok(match disk {
            ConfigDisk::CloudImage { name } => {
                let image_path = name.resolve_path(&self.common)?.canonicalize()?;
                let dest = PathBuf::from(format!("{}-cloud-disk.img", self.machine.name));
                if !dest.exists() {
                    std::fs::copy(image_path, &dest)?;
                    let mut perms = std::fs::metadata(&dest)?.permissions();
                    perms.set_readonly(false);
                    std::fs::set_permissions(&dest, perms)?;
                }
                DiskXml::new(
                    DiskDriverType::QCow2,
                    dest.canonicalize().unwrap().to_str().unwrap().to_owned(),
                    DiskDeviceType::Disk,
                    false,
                    "hda".to_string(),
                    TargetBus::Ide,
                )
            }
            ConfigDisk::ExistingDisk {
                path,
                driver_type,
                device_type,
                readonly,
            } => DiskXml::new(
                driver_type.clone(),
                path.canonicalize()
                    .with_context(|| "Finding virtual hard drive")?
                    .to_str()
                    .unwrap()
                    .to_owned(),
                device_type.clone(),
                *readonly,
                "hda".to_string(),
                TargetBus::Ide,
            ),
        })
    }

    fn cloud_init(&self, cloud_init: &CloudInit) -> anyhow::Result<DiskXml> {
        let dest = format!("{}-cloud-init.iso", self.machine.name);
        let dest = PathBuf::from(dest);
        genisoimage(
            dest.as_path(),
            &MetaData {
                instance_id: self.machine.name.clone(),
                local_hostname: self.machine.name.clone(),
            },
            &cloud_init.user_data,
        )?;
        Ok(DiskXml::new(
            DiskDriverType::Raw,
            dest.canonicalize()?.to_str().unwrap().to_owned(),
            DiskDeviceType::CdRom,
            true,
            "hdb".to_string(),
            TargetBus::Ide,
        ))
    }

    pub fn convert(&self) -> anyhow::Result<DomainXml> {
        let vnc = GraphicsXml::new("vnc".to_owned(), "-1".to_owned(), "yes".to_owned());
        let disk = self.create_virt_disk(&self.machine.disk)?;

        let mut builder = DomainXml::builder()
            .name(&self.machine.get_virt_name(&self.common))
            .memory(self.machine.memory_mb)
            .cpus(self.machine.cpus)
            .device(DeviceXML::Graphics(vnc))
            .device(DeviceXML::Disk(disk));

        match &self.machine.cloud_init {
            None => {}
            Some(cloud_init) => {
                let disk = self.cloud_init(cloud_init)?;
                builder = builder
                    .device(DeviceXML::Disk(disk))
                    .serial(Some("ds=nocloud;".to_string()));
            }
        }
        Ok(builder.build().unwrap())
    }
}
