use crate::config::convert::ConfigConverter;
use crate::config::{Config, ConfigMachine, GuestOperatingSystem};
use crate::virt_util::devices::DiskXml;
use crate::virt_util::{DiskDeviceType, DiskDriverType, TargetBus};
use crate::Common;
use anyhow::bail;
use anyhow::Context;
use serde::Serialize;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
struct MetaData {
    pub instance_id: String,
    pub local_hostname: String,
}

impl<'t> ConfigConverter<'t> {
    pub fn convert_cloud_init(&self, machine: &ConfigMachine) -> anyhow::Result<DiskXml> {
        let dest = PathBuf::from(format!("{}-cloud-init.iso", machine.name));
        let mut meta_data = self.cloud_init_metadata(&machine)?;
        let mut user_data = self.cloud_init_userdata(&machine)?;
        genisoimage(dest.as_path(), &mut meta_data, &mut user_data)?;
        Ok(DiskXml::new(
            DiskDriverType::Raw,
            dest.canonicalize()?.to_str().unwrap().to_owned(),
            DiskDeviceType::CdRom,
            true,
            "hdb".to_string(),
            TargetBus::Ide,
        ))
    }

    fn cloud_init_metadata(&self, machine: &ConfigMachine) -> anyhow::Result<Box<dyn ReadAndSeek>> {
        let instance_name = self.common.prepend_project(&machine.name);
        let file = machine
            .cloud_init
            .as_ref()
            .and_then(|c| c.meta_data_file.clone());
        Ok(match file {
            None => {
                let meta_data = MetaData {
                    instance_id: instance_name.clone(),
                    local_hostname: instance_name.clone(),
                };
                let data = serde_json::to_string(&meta_data).unwrap().into_bytes();
                Box::new(Cursor::new(data))
            }
            Some(f) => Box::new(
                std::fs::File::open(f).with_context(|| "Opening cloud-init meta_data file")?,
            ),
        })
    }

    fn cloud_init_userdata(&self, machine: &ConfigMachine) -> anyhow::Result<Box<dyn ReadAndSeek>> {
        let file = machine
            .cloud_init
            .as_ref()
            .and_then(|c| c.user_data_file.clone());
        Ok(match file {
            None => {
                let userdata = machine
                    .get_os()
                    .map(|m| m.generate_userdata(&self.config, &machine))
                    .unwrap_or("".to_ascii_lowercase());
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
    meta_data: &mut T,
    user_data: &mut T,
) -> anyhow::Result<()> {
    if output.exists() {
        log::trace!("replacing genisoimage");
        std::fs::remove_file(output)?;
    }
    let temp = Common::storage_location()?;

    let mut cmd = Command::new("genisoimage");
    cmd.arg("-output")
        .arg(output.to_str().unwrap())
        .arg("-volid")
        .arg("cidata")
        .arg("-joliet")
        .arg("-rock");

    let dest = temp.join("meta-data");
    meta_data.seek(SeekFrom::Start(0))?;
    let mut file = std::fs::File::create(&dest).with_context(|| "Creating cloud-init meta-data")?;
    std::io::copy(meta_data, &mut file)?;
    cmd.arg(dest.to_str().unwrap());

    let dest = temp.join("user-data");
    meta_data.seek(SeekFrom::Start(0))?;
    let mut file = std::fs::File::create(&dest).with_context(|| "Creating cloud-init user-data")?;
    std::io::copy(user_data, &mut file)?;
    cmd.arg(dest.to_str().unwrap());
    let mut user_data_perms = std::fs::metadata(&dest)?.permissions();
    user_data_perms.set_mode(0o777); // Needs to have write bit set - before going into ISO!
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

impl GuestOperatingSystem {
    pub fn generate_userdata(&self, _config: &Config, _machine: &ConfigMachine) -> String {
        match &self {
            // Cirros expects a script
            // https://github.com/cirros-dev/cirros/blob/master/doc/cirros-init.txt
            GuestOperatingSystem::Cirros => r#"
                passwd -d cirros
                echo "hello_world" >> /etc/hello
                "#
            .to_string(),
            GuestOperatingSystem::Ubuntu => "".to_string(),
        }
    }
}
