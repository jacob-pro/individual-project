use std::string::String;

use anyhow::anyhow;
use clap::Clap;
use log::LevelFilter;
use simple_logger::SimpleLogger;
use virt::connect::Connect;

use crate::config::Config;

mod config;
mod actions;
mod virt_util;

#[derive(Clap)]
#[clap(version = "1.0", author = "Jacob Halsey")]
struct Opts {
    #[clap(long, default_value = "kvm-compose.yaml")]
    input: String,
    #[clap(long, about = "Defaults to the current folder name")]
    project_name: Option<String>,
    #[clap(short, long)]
    verbosity: Option<String>,
    #[clap(subcommand)]
    sub_command: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Up,
    Down,
}

pub struct Common {
    hypervisor: Connect,
    config: Config,
    project: String,
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
        _ => Err(anyhow!("Unknown Log LevelFilter")),
    }
}

fn run_app() -> Result<(), anyhow::Error>{
    let opts: Opts = Opts::parse();
    let level = match &opts.verbosity {
        None => LevelFilter::Info,
        Some(x) => log_level(x)?,
    };
    SimpleLogger::new().with_level(level).init().unwrap();

    let config = Config::load_from_file(opts.input)?;
    log::trace!("Connecting to QEMU");
    let conn =  Connect::open("qemu:///session")?;

    let project_name = match opts.project_name {
        None => {
            let path = std::env::current_dir()?;
            path.iter().last().unwrap().to_str().unwrap().to_owned()
        }
        Some(x) => {x}
    };

    let common = Common { hypervisor: conn, config, project: project_name };
    match opts.sub_command {
        SubCommand::Up => {actions::up(common)?}
        SubCommand::Down => {actions::down(common)?}
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
