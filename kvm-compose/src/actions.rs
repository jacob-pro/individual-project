use crate::config::convert::ConfigConverter;
use crate::network::ip::{Dhclient, Ip};
use crate::network::ovs::OvsVsctl;
use crate::Common;
use anyhow::Context;
use std::path::PathBuf;
use virt::domain::Domain;

pub fn up(common: Common) -> anyhow::Result<()> {
    let converter = ConfigConverter::new(&common, &common.config);

    for bridge in &common.config.bridges {
        let bridge_name = common.prepend_project(&bridge.name);
        if !OvsVsctl::br_exists(&bridge_name)? {
            log::info!("Creating bridge {}", bridge_name);
            OvsVsctl::add_br(&bridge_name)?;
            for interface in &bridge.connect_external_interfaces {
                log::trace!("Attaching {} to {}", interface, bridge_name);
                OvsVsctl::add_port(&bridge_name, interface, None)?;
                Ip::addr_flush_dev(interface)?;
            }
            if bridge.enable_dhcp_client {
                log::trace!("Launching DHCP client for {}", bridge_name);
                Dhclient::run(&bridge_name)?;
            }
            match &bridge.protocol {
                None => {}
                Some(protocol) => {
                    log::trace!("Setting bridge {} protocol to {}", bridge_name, protocol);
                    OvsVsctl::set_bridge_protocol(&bridge_name, protocol)?;
                }
            }
            match &bridge.controller {
                None => {}
                Some(controller) => {
                    log::trace!(
                        "Setting bridge {} controller to {}",
                        bridge_name,
                        controller
                    );
                    OvsVsctl::set_controller(&bridge_name, controller)?;
                }
            }
        } else {
            log::info!("Bridge {} already exists", bridge_name);
        }
    }

    for machine in &common.config.machines {
        match domain_lookup_by_name(&common, &machine.get_virt_name(&common))? {
            None => {
                log::info!("Creating machine {}", machine.name);
                let xml = converter.convert_machine(&machine)?.to_xml();
                log::trace!("{}", xml);
                let d = Domain::define_xml(&common.hypervisor, xml.as_str())
                    .with_context(|| format!("Failed to define_xml for {}", machine.name))?;

                match d.create() {
                    Ok(_) => {}
                    Err(e) => {
                        d.undefine().ok();
                        return Err(e)
                            .with_context(|| format!("Failed to start vm {}", machine.name));
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
        let bridge_name = common.prepend_project(&bridge.name);
        if OvsVsctl::br_exists(&bridge_name)? {
            log::info!("Removing bridge {}", bridge_name);
            OvsVsctl::del_br(&bridge_name)?;
            for interface in &bridge.connect_external_interfaces {
                log::trace!("Re-launching DHCP client for {}", interface);
                Dhclient::run(interface)?; // Restore address to interface
            }
        }
    }

    Ok(())
}

fn domain_lookup_by_name(c: &Common, name: &str) -> anyhow::Result<Option<Domain>> {
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
