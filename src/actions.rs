use crate::Common;
use anyhow::Context;
use virt::domain::Domain;

pub fn up(common: Common) -> anyhow::Result<()> {
    for machine in &common.config.machines {

        match domain_lookup_by_name(&common, &machine.virt_name(&common))? {
            None => {
                log::info!("Creating machine {}", machine.name);
                let xml = machine.to_virt_domain(&common)?.to_xml();
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
        match domain_lookup_by_name(&common, &machine.virt_name(&common))? {
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
