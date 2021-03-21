use crate::Common;
use std::path::Path;
use std::process::Command;
use anyhow::Context;

pub fn genisoimage(
    output: &Path,
    meta_data: Option<&Path>,
    user_data: Option<&Path>,
) -> anyhow::Result<()> {
    let temp = Common::storage_location()?;

    let mut cmd = Command::new("genisoimage");
    cmd.arg("-output")
        .arg(output.to_str().unwrap())
        .arg("-volid")
        .arg("cidata")
        .arg("-joliet")
        .arg("-rock");

    match meta_data {
        None => {}
        Some(meta_data) => {
            let dest = temp.join("meta-data");
            std::fs::copy(meta_data, &dest).with_context(|| "Copying cloud-init meta-data")?;
            cmd.arg(dest.to_str().unwrap());
        }
    }
    match user_data {
        None => {}
        Some(user_data) => {
            let dest = temp.join("user-data");
            std::fs::copy(user_data, &dest).with_context(|| "Copying cloud-init user-data")?;
            cmd.arg(dest.to_str().unwrap());
        }
    }
    cmd.output()?;
    assert!(output.is_file());
    Ok(())
}
