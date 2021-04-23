use anyhow::bail;
use std::process::Command;

pub struct OvsVsctl;

impl OvsVsctl {
    pub fn add_br<T: AsRef<str>>(name: T) -> anyhow::Result<()> {
        let output = Command::new("sudo")
            .arg("ovs-vsctl")
            .arg("add-br")
            .arg(name.as_ref())
            .output()?;
        if !output.status.success() {
            let std_err = std::str::from_utf8(&output.stderr)?;
            bail!("{}", std_err);
        }
        Ok(())
    }

    pub fn br_exists<T: AsRef<str>>(name: T) -> anyhow::Result<bool> {
        let output = Command::new("sudo")
            .arg("ovs-vsctl")
            .arg("br-exists")
            .arg(name.as_ref())
            .output()?;
        if !output.status.success() {
            match output.status.code() {
                Some(x) if x == 2 => return Ok(false),
                _ => {
                    let std_err = std::str::from_utf8(&output.stderr)?;
                    bail!("{}", std_err);
                }
            }
        }
        Ok(true)
    }

    pub fn del_br<T: AsRef<str>>(name: T) -> anyhow::Result<()> {
        let output = Command::new("sudo")
            .arg("ovs-vsctl")
            .arg("del-br")
            .arg(name.as_ref())
            .output()?;
        if !output.status.success() {
            let std_err = std::str::from_utf8(&output.stderr)?;
            bail!("{}", std_err);
        }
        Ok(())
    }

    pub fn add_port<T: AsRef<str>>(name: T, interface: T, tag: Option<u16>) -> anyhow::Result<()> {
        let mut cmd = Command::new("sudo");
        cmd.arg("ovs-vsctl")
            .arg("add-port")
            .arg(name.as_ref())
            .arg(interface.as_ref());
        match tag {
            None => {}
            Some(tag) => {
                cmd.arg(format!("tag={}", tag));
            }
        };
        let output = cmd.output()?;
        if !output.status.success() {
            let std_err = std::str::from_utf8(&output.stderr)?;
            bail!("{}", std_err);
        }
        Ok(())
    }

    pub fn set_controller<T: AsRef<str>, Y: AsRef<str>>(
        bridge: T,
        controller: Y,
    ) -> anyhow::Result<()> {
        let output = Command::new("sudo")
            .arg("ovs-vsctl")
            .arg("set-controller")
            .arg(bridge.as_ref())
            .arg(controller.as_ref())
            .output()?;
        if !output.status.success() {
            let std_err = std::str::from_utf8(&output.stderr)?;
            bail!("{}", std_err);
        }
        Ok(())
    }
}
