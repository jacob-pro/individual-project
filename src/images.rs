use serde::{Serialize, Deserialize};
use std::path::{PathBuf, Path};
use crate::Common;
use indicatif::ProgressBar;
use std::fs::File;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
#[allow(non_camel_case_types)]
pub enum OnlineCloudImage {
    Ubuntu_18_04,
    Cirros_0_5_1,
}

impl OnlineCloudImage {

    fn get_url(&self) -> &str {
        match &self {
            OnlineCloudImage::Ubuntu_18_04 => "https://cloud-images.ubuntu.com/bionic/current/bionic-server-cloudimg-amd64.img",
            OnlineCloudImage::Cirros_0_5_1 => "http://download.cirros-cloud.net/0.5.1/cirros-0.5.1-x86_64-disk.img"
        }
    }

    pub fn resolve_path(&self, common: &Common) -> anyhow::Result<PathBuf> {
        let mut name = Common::storage_location()?;
        name.push(format!("{}.img", serde_plain::to_string(self).unwrap()));
        if !name.is_file() {
            common.confirm_continue(
                format!("Download image {:?} to {:?}?", self, name).as_str());
            download_file(self.get_url(), &name)?;
        }
        Ok(name)
    }

}

fn download_file(url: &str, destination: &Path) -> anyhow::Result<()>{
    println!("Downloading {}", url);
    let dest = tempfile::NamedTempFile::new().unwrap();
    let mut res = reqwest::blocking::get(url)?;
    let len = res.content_length().unwrap_or(u64::MAX);
    let bar = ProgressBar::new(len);
    let mut writer = WrappedWriter {
        dest: dest.as_file(),
        callback: |w| bar.set_position(w),
        written: 0
    };
    res.copy_to(&mut writer)?;
    bar.finish();
    dest.persist(destination)?;
    Ok(())
}

struct WrappedWriter<'t, C> {
    dest: &'t File,
    callback: C,
    written: u64,
}

impl<'t, C> std::io::Write for WrappedWriter<'t, C>
    where C: Fn(u64)
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let res = self.dest.write(buf);
        match res {
            Ok(w) => {
                self.written = self.written + w as u64;
                (self.callback)(self.written);
            }
            Err(_) => {}
        }
        res
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.dest.flush()
    }
}