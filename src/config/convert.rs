use crate::config::{Config, ConfigDisk, ConfigInterface, ConfigMachine};
use crate::virt_util::devices::{DeviceXML, DiskXml};
use crate::virt_util::devices::{GraphicsXml, InterfaceXml};
use crate::virt_util::domain::DomainXml;
use crate::virt_util::{DiskDeviceType, DiskDriverType, TargetBus};
use crate::Common;
use anyhow::Context;
use std::path::PathBuf;

#[derive(new)]
pub struct ConfigConverter<'t> {
    pub common: &'t Common,
    pub config: &'t Config,
}

impl<'t> ConfigConverter<'t> {
    fn convert_disk(&self, machine: &ConfigMachine, disk: &ConfigDisk) -> anyhow::Result<DiskXml> {
        Ok(match disk {
            ConfigDisk::CloudImage { name } => {
                let image_path = name.resolve_path(&self.common)?.canonicalize()?;
                let dest = PathBuf::from(format!("{}-cloud-disk.img", machine.name));
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

    fn convert_interface(&self, interface: &ConfigInterface) -> InterfaceXml {
        InterfaceXml::new(self.common.prepend_project(&interface.bridge))
    }

    pub fn convert_machine(&self, machine: &ConfigMachine) -> anyhow::Result<DomainXml> {
        let vnc = GraphicsXml::new("vnc".to_owned(), "-1".to_owned(), "yes".to_owned());
        let disk = self.convert_disk(&machine, &machine.disk)?;
        let cloud_init_disk = self.convert_cloud_init(&machine)?;

        let mut builder = DomainXml::builder()
            .name(&machine.get_virt_name(&self.common))
            .memory(machine.memory_mb)
            .cpus(machine.cpus)
            .device(DeviceXML::Graphics(vnc))
            .device(DeviceXML::Disk(disk))
            .device(DeviceXML::Disk(cloud_init_disk))
            .serial(Some("ds=nocloud;".to_string()));

        for i in &machine.interfaces {
            builder = builder.device(DeviceXML::Interface(self.convert_interface(i)));
        }

        Ok(builder.build().unwrap())
    }
}
