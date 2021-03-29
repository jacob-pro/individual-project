use anyhow::bail;
use std::process::Command;

pub struct Bridge;

impl Bridge {
    pub fn add<T: AsRef<str>>(name: T) -> anyhow::Result<()> {
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

    pub fn exists<T: AsRef<str>>(name: T) -> anyhow::Result<bool> {
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

    pub fn delete<T: AsRef<str>>(name: T) -> anyhow::Result<()> {
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
}
