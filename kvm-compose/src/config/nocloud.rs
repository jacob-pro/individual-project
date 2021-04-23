use crate::assets::Assets;
use crate::config::convert::ConfigConverter;
use crate::config::{ConfigDisk, ConfigMachine};
use crate::virt_util::devices::DiskXml;
use crate::virt_util::{DiskDeviceType, DiskDriverType, TargetBus};
use crate::Common;
use anyhow::bail;
use anyhow::Context;
use serde::Serialize;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

// NoCloud supports three keys: [local-hostname, instance-id, seedfrom]
// Other keys will show up under `cloud-init query ds.meta_data.key`
// Or on Cirros `cirros-query get key`
#[derive(Serialize, Clone, Debug)]
struct NoCloudMetadata {
    #[serde(rename = "instance-id")]
    instance_id: String,
    #[serde(rename = "local-hostname")]
    local_hostname: String,
    public_ssh_key: String,
    run_script: String,
}

impl<'t> ConfigConverter<'t> {
    pub fn convert_cloud_init(&self, machine: &ConfigMachine) -> anyhow::Result<DiskXml> {
        let instance_name = self.common.prepend_project(&machine.name);
        let script = match &machine.run_script {
            None => "".to_owned(),
            Some(path) => std::fs::read_to_string(path).with_context(|| "Reading run_script")?,
        };
        let meta_data = NoCloudMetadata {
            instance_id: instance_name.clone(),
            local_hostname: instance_name.clone(),
            public_ssh_key: self.config.ssh_public_key.clone(),
            run_script: script,
        };

        let init_type = if let ConfigDisk::CloudImage { name, .. } = &machine.disk {
            name.get_cloud_init_type()
        } else {
            CloudInitType::CloudInit
        };
        let user_data = init_type.generate_userdata();

        let dest = PathBuf::from(format!("{}-cloud-init.iso", machine.name));
        genisoimage(dest.as_path(), &meta_data, &user_data)?;
        Ok(DiskXml::new(
            DiskDriverType::Raw,
            dest.canonicalize()?.to_str().unwrap().to_owned(),
            DiskDeviceType::CdRom,
            true,
            "hdb".to_string(),
            TargetBus::Ide,
        ))
    }
}

fn genisoimage(
    output: &Path,
    meta_data: &NoCloudMetadata,
    user_data: &Vec<u8>,
) -> anyhow::Result<()> {
    if output.exists() {
        log::trace!("replacing genisoimage");
        std::fs::remove_file(output)?;
    }
    let kvm_appdata = Common::storage_location()?;

    let mut cmd = Command::new("genisoimage");
    cmd.arg("-output")
        .arg(output.to_str().unwrap())
        .arg("-volid")
        .arg("cidata")
        .arg("-joliet")
        .arg("-rock");

    let dest = kvm_appdata.join("meta-data");
    let mut file = std::fs::File::create(&dest).with_context(|| "Creating cloud-init meta-data")?;
    file.write_all(serde_json::to_string(&meta_data).unwrap().as_bytes())?;
    cmd.arg(dest.to_str().unwrap());

    let dest = kvm_appdata.join("user-data");
    let mut file = std::fs::File::create(&dest).with_context(|| "Creating cloud-init user-data")?;
    file.write_all(user_data)?;
    cmd.arg(dest.to_str().unwrap());
    // User data might be a script, so it should have write bit set before going into ISO
    let mut user_data_perms = std::fs::metadata(&dest)?.permissions();
    user_data_perms.set_mode(0o700);
    std::fs::set_permissions(&dest, user_data_perms)?;

    let cmd_output = cmd.output()?;
    if !cmd_output.status.success() {
        let std_err = std::str::from_utf8(&cmd_output.stderr)?;
        bail!("{}", std_err);
    }

    let mut iso_perms = std::fs::metadata(&output)?.permissions();
    iso_perms.set_readonly(true);
    std::fs::set_permissions(&output, iso_perms)?;
    Ok(())
}

#[derive(Clone, Debug)]
pub enum CloudInitType {
    CirrosInit,
    CloudInit,
}

impl CloudInitType {
    fn generate_userdata(&self) -> Vec<u8> {
        match &self {
            CloudInitType::CirrosInit => Assets::get("cirros_init.sh").unwrap().into_owned(),
            CloudInitType::CloudInit => Assets::get("cloud_init.yaml").unwrap().into_owned(),
        }
    }
}
