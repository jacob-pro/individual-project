use crate::config::convert::MachineToDomainConverter;
use crate::ovs::Bridge;
use crate::Common;
use anyhow::Context;
use std::path::PathBuf;
use virt::domain::Domain;

pub fn up(common: Common) -> anyhow::Result<()> {
    for bridge in &common.config.bridges {
        let name = common.prepend_project(&bridge.name);
        if !Bridge::exists(&name)? {
            log::info!("Creating bridge {}", name);
            Bridge::add(&name)?;
        } else {
            log::info!("Bridge {} already exists", name);
        }
    }

    for machine in &common.config.machines {
        match domain_lookup_by_name(&common, &machine.get_virt_name(&common))? {
            None => {
                log::info!("Creating machine {}", machine.name);
                let xml = MachineToDomainConverter::new(&common, &machine)
                    .convert()?
                    .to_xml();
                log::trace!("{}", xml);
                let d = Domain::define_xml(&common.hypervisor, xml.as_str())
                    .with_context(|| format!("Failed to define_xml for {}", machine.name))?;

                match d.create() {
                    Ok(_) => {}
                    Err(e) => {
                        d.undefine().ok();
                        return Err(e).with_context(|| format!("Failed to start vm {}", machine.name))
                    }
                }

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
        match domain_lookup_by_name(&common, &machine.get_virt_name(&common))? {
            None => {}
            Some(d) => {
                if d.is_active()? {
                    log::info!("Stopping machine {}", machine.name);
                    d.destroy()?;
                }
                log::info!("Removing machine {}", machine.name);
                d.undefine()?;
            }
        }

        let cloud_init = PathBuf::from(format!("{}-cloud-init.iso", machine.name));
        if cloud_init.exists() && cloud_init.is_file() {
            log::info!("Removing machine {} cloud-init.iso", machine.name);
            std::fs::remove_file(cloud_init)?;
        }

        let cloud_disk = PathBuf::from(format!("{}-cloud-disk.img", machine.name));
        if cloud_disk.exists() && cloud_disk.is_file() {
            log::info!("Removing machine {} cloud-disk.img", machine.name);
            std::fs::remove_file(cloud_disk)?;
        }
    }

    for bridge in &common.config.bridges {
        let name = common.prepend_project(&bridge.name);
        if Bridge::exists(&name)? {
            log::info!("Removing bridge {}", name);
            Bridge::delete(&name)?;
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
