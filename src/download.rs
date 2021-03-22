use indicatif::ProgressBar;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::os::unix::fs::PermissionsExt;

pub fn download_file(url: &str, destination: &Path, mode: u32) -> anyhow::Result<File> {
    log::info!("Downloading {}", url);
    let mut dest = tempfile::NamedTempFile::new().unwrap();
    let mut res = reqwest::blocking::get(url)?;
    let len = res.content_length().unwrap_or(u64::MAX);
    let bar = ProgressBar::new(len);
    let mut writer = CountingWriter::new(dest.as_file_mut(), |w| bar.set_position(w));
    res.copy_to(&mut writer)?;
    bar.finish();
    let f = dest.persist(destination)?;
    let metadata = f.metadata()?;
    let mut permissions = metadata.permissions();
    permissions.set_mode(mode);
    f.set_permissions(permissions)?;
    Ok(f)
}

pub struct CountingWriter<'t, W, C> {
    dest: &'t mut W,
    callback: C,
    written: u64,
}

impl<'t, W, C> CountingWriter<'t, W, C> {
    pub fn new(dest: &'t mut W, callback: C) -> Self {
        Self {
            dest,
            callback,
            written: 0,
        }
    }
}

impl<'t, W, C> Write for CountingWriter<'t, W, C>
where
    C: Fn(u64),
    W: Write,
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
