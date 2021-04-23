use anyhow::bail;
use std::process::Command;

pub struct QemuImg {}

impl QemuImg {
    pub fn resize<T: AsRef<str>, Y: AsRef<str>>(disk: T, arg2: Y) -> anyhow::Result<()> {
        let output = Command::new("sudo")
            .arg("qemu-img")
            .arg("resize")
            .arg(disk.as_ref())
            .arg(arg2.as_ref())
            .output()?;
        if !output.status.success() {
            let std_err = std::str::from_utf8(&output.stderr)?;
            bail!("{}", std_err);
        }
        Ok(())
    }
}
