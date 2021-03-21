use crate::Common;
use anyhow::Context;
use std::path::Path;
use std::process::Command;

pub fn genisoimage(output: &Path, meta_data: &Path, user_data: &Path) -> anyhow::Result<()> {
    let temp = Common::storage_location()?;

    let mut cmd = Command::new("genisoimage");
    cmd.arg("-output")
        .arg(output.to_str().unwrap())
        .arg("-volid")
        .arg("cidata")
        .arg("-joliet")
        .arg("-rock");

    let dest = temp.join("meta-data");
    std::fs::copy(meta_data, &dest).with_context(|| "Copying cloud-init meta-data")?;
    cmd.arg(dest.to_str().unwrap());

    let dest = temp.join("user-data");
    std::fs::copy(user_data, &dest).with_context(|| "Copying cloud-init user-data")?;
    cmd.arg(dest.to_str().unwrap());

    cmd.output()?;
    assert!(output.is_file());
    Ok(())
}
