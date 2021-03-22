use crate::Common;
use anyhow::bail;
use anyhow::Context;
use serde::Serialize;
use std::io::Write;
use std::path::Path;
use std::process::Command;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct MetaData {
    pub instance_id: String,
    pub local_hostname: String,
}

pub fn genisoimage(output: &Path, meta_data: &MetaData, user_data: &Path) -> anyhow::Result<()> {
    let temp = Common::storage_location()?;

    let mut cmd = Command::new("genisoimage");
    cmd.arg("-output")
        .arg(output.to_str().unwrap())
        .arg("-volid")
        .arg("cidata")
        .arg("-joliet")
        .arg("-rock");

    let dest = temp.join("meta-data");
    let mut dest_file = std::fs::File::create(&dest)?;
    dest_file
        .write_all(serde_json::to_string(&meta_data).unwrap().as_bytes())
        .unwrap();
    cmd.arg(dest.to_str().unwrap());

    let dest = temp.join("user-data");
    std::fs::copy(user_data, &dest).with_context(|| "Copying cloud-init user-data")?;
    cmd.arg(dest.to_str().unwrap());

    let cmd_output = cmd.output()?;
    if !cmd_output.status.success() {
        let std_err = std::str::from_utf8(&cmd_output.stderr)?;
        bail!("{}", std_err);
    }

    let mut perms = std::fs::metadata(&output)?.permissions();
    perms.set_readonly(true);
    std::fs::set_permissions(&output, perms)?;
    Ok(())
}
