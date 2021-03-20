use crate::virt_util::devices::{DeviceXML, Disk, DiskDevice, Graphics};
use crate::virt_util::domain::DomainXml;
use crate::Common;
use anyhow::Context;
use virt::domain::Domain;

pub fn up(common: Common) -> anyhow::Result<()> {
    for machine in &common.config.machines {
        let name = format!("{}-{}", common.project, machine.name);

        match domain_lookup_by_name(&common, &name)? {
            None => {
                log::info!("Creating machine {}", machine.name);

                let image_path = machine.get_image_path(&common)?.canonicalize()?;
                let cdrom = Disk::new(
                    "qcow2".to_owned(),
                    image_path.to_str().unwrap().to_owned(),
                    DiskDevice::CdRom,
                    true,
                    "hdc".to_string(),
                );
                let vnc = Graphics::new("vnc".to_owned(), "-1".to_owned(), "yes".to_owned());

                let xml = DomainXml::builder()
                    .name(&name)
                    .memory(machine.memory)
                    .cpus(machine.cpus)
                    .device(DeviceXML::Disk(cdrom))
                    .device(DeviceXML::Graphics(vnc))
                    .build()
                    .unwrap()
                    .to_xml();
                log::trace!("{}", xml);

                let d = Domain::define_xml(&common.hypervisor, xml.as_str())
                    .with_context(|| format!("Failed to define_xml for {}", machine.name))?;
                d.create()
                    .with_context(|| format!("Failed to start vm {}", machine.name))?;
            }
            Some(d) => {
                if !d.is_active()? {
                    log::info!("{} already exists, starting", machine.name);
                    d.create()
                        .with_context(|| format!("Failed to start vm {}", machine.name))?;
                } else {
                    log::info!("{} already exists, already running", machine.name);
                }
            }
        }
    }

    Ok(())
}

pub fn down(common: Common) -> anyhow::Result<()> {
    for machine in &common.config.machines {
        let name = format!("{}-{}", common.project, machine.name);
        match domain_lookup_by_name(&common, &name)? {
            None => {}
            Some(d) => {
                if d.is_active()? {
                    log::trace!("Stopping machine {}", machine.name);
                    d.destroy()?;
                }
                log::info!("Removing machine {}", machine.name);
                d.undefine()?;
            }
        }
    }

    Ok(())
}

pub fn domain_lookup_by_name(c: &Common, name: &str) -> anyhow::Result<Option<Domain>> {
    match Domain::lookup_by_name(&c.hypervisor, name) {
        Ok(x) => Ok(Some(x)),
        // VIR_ERR_NO_DOMAIN
        Err(e) => {
            if e.code == 0x2a {
                Ok(None)
            } else {
                Err(e.into())
            }
        }
    }
}
