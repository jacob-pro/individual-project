use crate::config::convert::ConfigConverter;
use crate::config::{Config, ConfigDisk, ConfigMachine};
use crate::virt_util::devices::DiskXml;
use crate::virt_util::{DiskDeviceType, DiskDriverType, TargetBus};
use crate::Common;
use anyhow::bail;
use anyhow::Context;
use serde::Serialize;
use std::borrow::Cow;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

// NoCloud supports three keys: [local-hostname, instance-id, seedfrom]
#[derive(Serialize, Clone, Debug)]
struct NoCloudMetadata {
    #[serde(rename = "instance-id")]
    instance_id: String,
    #[serde(rename = "local-hostname")]
    local_hostname: String,
    public_ssh_key: String,
}

impl<'t> ConfigConverter<'t> {
    pub fn convert_cloud_init(&self, machine: &ConfigMachine) -> anyhow::Result<DiskXml> {
        let dest = PathBuf::from(format!("{}-cloud-init.iso", machine.name));
        let instance_name = self.common.prepend_project(&machine.name);
        let meta_data = NoCloudMetadata {
            instance_id: instance_name.clone(),
            local_hostname: instance_name.clone(),
            public_ssh_key: self.config.ssh_public_key.clone(),
        };
        let mut user_data = self.cloud_init_userdata(&machine)?;
        genisoimage(dest.as_path(), &meta_data, &mut user_data)?;
        Ok(DiskXml::new(
            DiskDriverType::Raw,
            dest.canonicalize()?.to_str().unwrap().to_owned(),
            DiskDeviceType::CdRom,
            true,
            "hdb".to_string(),
            TargetBus::Ide,
        ))
    }

    fn cloud_init_userdata(&self, machine: &ConfigMachine) -> anyhow::Result<Box<dyn ReadAndSeek>> {
        Ok(match &machine.init_script {
            None => {
                let init_type = if let ConfigDisk::CloudImage { name } = &machine.disk {
                    name.get_cloud_init_type()
                } else {
                    CloudInitType::CloudInit
                };
                let userdata = init_type.generate_userdata(&self.config, &machine);
                Box::new(Cursor::new(userdata.into_bytes()))
            }
            Some(f) => Box::new(
                std::fs::File::open(f).with_context(|| "Opening cloud-init meta_data file")?,
            ),
        })
    }
}

trait ReadAndSeek: Read + Seek {}
impl<T: Read + Seek> ReadAndSeek for T {}

fn genisoimage<T: ReadAndSeek>(
    output: &Path,
    meta_data: &NoCloudMetadata,
    user_data: &mut T,
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
    user_data.seek(SeekFrom::Start(0))?;
    let mut file = std::fs::File::create(&dest).with_context(|| "Creating cloud-init user-data")?;
    std::io::copy(user_data, &mut file)?;
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
    pub fn generate_userdata(&self, config: &Config, _machine: &ConfigMachine) -> String {
        match &self {
            // Cirros-init only seems to support / expect a script
            // https://github.com/cirros-dev/cirros/blob/master/doc/cirros-init.txt
            CloudInitType::CirrosInit => format!(
                r#"
                #!
                mkdir -p /home/cirros/.ssh
                echo {} > /home/cirros/.ssh/authorized_keys
                chmod 644 /home/cirros/.ssh/authorized_keys
                "#,
                shell_escape::unix::escape(Cow::from(&config.ssh_public_key))
            ),
            // https://help.ubuntu.com/community/CloudInit
            CloudInitType::CloudInit => format!(
                r#"
                #cloud-config
                chpasswd:
                  list: |
                    ubuntu:password
                  expire: False
                "#
            ),
        }
    }
}
