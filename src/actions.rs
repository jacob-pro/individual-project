use crate::virt_util::devices::{DeviceXML, Disk, DiskDevice};
use crate::virt_util::domain::DomainXml;
use crate::Common;
use anyhow::Context;
use virt::domain::Domain;

pub fn up(common: Common) -> anyhow::Result<()> {
    for machine in &common.config.machines {
        let name = format!("{}-{}", common.project, machine.name);

        let image_path = machine.get_image_path(&common)?.canonicalize()?;
        let cdrom = Disk::new(
            "raw".to_owned(),
            image_path.to_str().unwrap().to_owned(),
            DiskDevice::CdRom,
            true,
            "hdc".to_string(),
        );

        let xml = DomainXml::builder()
            .name(&name)
            .memory(machine.memory)
            .cpus(machine.cpus)
            .device(DeviceXML::Disk(cdrom))
            .build()
            .unwrap()
            .to_xml();

        log::trace!("{}", xml);
        if domain_lookup_by_name(&common, &name)?.is_some() {
            log::info!("{} already exists, skipping", machine.name);
        } else {
            log::info!("Creating machine {}", machine.name);
            Domain::define_xml(&common.hypervisor, xml.as_str())
                .with_context(|| format!("Failed to define_xml for router {}", machine.name))?;
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
