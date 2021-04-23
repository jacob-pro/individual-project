use anyhow::bail;
use std::process::Command;

pub struct Ip {}

impl Ip {
    pub fn addr_flush_dev<T: AsRef<str>>(device: T) -> anyhow::Result<()> {
        let output = Command::new("sudo")
            .arg("ip")
            .arg("addr")
            .arg("flush")
            .arg("dev")
            .arg(device.as_ref())
            .output()?;
        if !output.status.success() {
            let std_err = std::str::from_utf8(&output.stderr)?;
            bail!("{}", std_err);
        }
        Ok(())
    }
}

pub struct Dhclient {}

impl Dhclient {
    pub fn run<T: AsRef<str>>(device: T) -> anyhow::Result<()> {
        let output = Command::new("sudo")
            .arg("dhclient")
            .arg(device.as_ref())
            .output()?;
        if !output.status.success() {
            let std_err = std::str::from_utf8(&output.stderr)?;
            bail!("{}", std_err);
        }
        Ok(())
    }
}
