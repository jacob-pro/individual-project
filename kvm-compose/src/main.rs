#[macro_use]
extern crate derive_new;

use crate::config::images::OnlineCloudImage;
use crate::config::Config;
use anyhow::anyhow;
use clap::Clap;
use directories::UserDirs;
use log::LevelFilter;
use simple_logger::SimpleLogger;
use std::path::PathBuf;
use std::string::String;
use virt::connect::Connect;

mod actions;
mod assets;
mod config;
mod download;
mod network;
mod virt_util;

#[derive(Clap)]
#[clap(version = "1.0", author = "Jacob Halsey")]
struct Opts {
    #[clap(long, default_value = "kvm-compose.yaml", about = "Configuration file")]
    input: String,
    #[clap(long, about = "Defaults to the current folder name")]
    project_name: Option<String>,
    #[clap(short, long)]
    verbosity: Option<String>,
    #[clap(long, about = "Suppress (accept) continue prompts")]
    no_ask: bool,
    #[clap(subcommand)]
    sub_command: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(about = "Create all virtual devices in the current configuration")]
    Up,
    #[clap(about = "Destroy all virtual devices in the current configuration")]
    Down,
    #[clap(about = "List supported cloud images")]
    CloudImages,
    #[clap(about = "Configure individual machine")]
    Machine(MachineCmd),
}

/// A subcommand for controlling testing
#[derive(Clap)]
pub struct MachineCmd {
    #[clap(long)]
    machine: String,
    #[clap(subcommand)]
    sub_command: MachineSubCommand,
}

#[derive(Clap)]
pub enum MachineSubCommand {
    #[clap(about = "Create this machine")]
    Up,
    #[clap(about = "Destroy this machine")]
    Down,
}

pub struct Common {
    hypervisor: Connect,
    config: Config,
    project: String,
    no_ask: bool,
}

impl Common {
    pub fn storage_location() -> anyhow::Result<PathBuf> {
        let mut p = UserDirs::new().unwrap().home_dir().to_owned();
        p.push(".kvm-compose");
        std::fs::create_dir_all(&p)?;
        Ok(p)
    }

    pub fn confirm_continue(&self, message: &str) {
        if !self.no_ask {
            if !casual::confirm(message) {
                std::process::exit(0);
            }
        }
    }

    pub fn prepend_project<T: AsRef<str>>(&self, t: T) -> String {
        format!("{}-{}", self.project, t.as_ref())
    }
}

impl Drop for Common {
    fn drop(&mut self) {
        match self.hypervisor.close() {
            Ok(_) => log::trace!("Disconnected from QEMU"),
            Err(e) => log::warn!("Failed to disconnect QEMU gracefully {}", e),
        };
    }
}

fn log_level(s: &str) -> anyhow::Result<LevelFilter> {
    match s.to_lowercase().as_str() {
        "error" => Ok(LevelFilter::Error),
        "warn" => Ok(LevelFilter::Warn),
        "info" => Ok(LevelFilter::Info),
        "trace" => Ok(LevelFilter::Trace),
        _ => Err(anyhow!("Unknown Log LevelFilter {}", s)),
    }
}

fn run_app() -> Result<(), anyhow::Error> {
    let opts: Opts = Opts::parse();
    let mut e = None;
    let level = match &opts.verbosity {
        None => LevelFilter::Info,
        Some(x) => match log_level(x) {
            Ok(l) => l,
            Err(err) => {
                e = Some(err);
                LevelFilter::Info
            }
        },
    };
    SimpleLogger::new()
        .with_level(LevelFilter::Error)
        .with_module_level(std::module_path!(), level)
        .init()
        .unwrap();
    e.map(|e| log::warn!("{}", e));

    match opts.sub_command {
        SubCommand::CloudImages => {
            OnlineCloudImage::print_image_list();
            return Ok(());
        }
        _ => {}
    }

    let config = Config::load_from_file(opts.input)?;
    let project_name = match opts.project_name {
        None => {
            let path = std::env::current_dir()?;
            path.iter().last().unwrap().to_str().unwrap().to_owned()
        }
        Some(x) => x,
    };
    log::trace!("Project name: {}", project_name);

    log::trace!("Connecting to qemu:///system");
    let conn = Connect::open("qemu:///system")?;

    let common = Common {
        hypervisor: conn,
        config,
        project: project_name,
        no_ask: opts.no_ask,
    };
    match opts.sub_command {
        SubCommand::Up => actions::up(common)?,
        SubCommand::Down => actions::down(common)?,
        SubCommand::Machine(machine) => actions::machine(common, machine)?,
        _ => {}
    }
    Ok(())
}

fn main() {
    virt::error::Error::clear_error_func();
    std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(err) => {
            log::error!("{:#}", err);
            1
        }
    });
}
