mod config;
mod error;

use std::{path::Path, process::ExitCode};
use clap::{Parser, Subcommand};
use config::Config;

#[derive(Clone, PartialEq, Subcommand)]
enum Commands {
    Init,
    Regen,
    Empty,
    Exclude
}

#[derive(Parser)]
struct CLI {
    #[command(subcommand)]
    cmd: Commands
}

fn init() -> error::Result<()> {
    if Path::new("./TZ.yml").exists() {
        return error::error(error::ErrorCode::CONFIG_ALREADY_EXISTS, None)
    }
    error::error(error::ErrorCode::INTERNAL, Some("init unimplemented".to_string()))
}

fn regen(config: Config) -> error::Result<()> {
    error::error(error::ErrorCode::INTERNAL, Some("regen unimplemented".to_string()))
}

fn empty(config: Config) -> error::Result<()> {
    if config.ephemeral_heading.is_none() {
        return error::error(error::ErrorCode::EPHEMERAL_DISABLED, None)
    }
    error::error(error::ErrorCode::INTERNAL, Some("empty unimplemented".to_string()))
}

fn exclude(config: Config) -> error::Result<()> {
    if config.ephemeral_heading.is_none() {
        return error::error(error::ErrorCode::EPHEMERAL_DISABLED, None)
    }
    error::error(error::ErrorCode::INTERNAL, Some("exclude unimplemented".to_string()))
}

fn main() -> ExitCode {
    let args = CLI::parse();
    let config = match config::Config::new(&Path::new("./TZ.yml")) {
        Ok(v) => v,
        Err(e) => {
            if args.cmd != Commands::Init {
                eprint!("{e}");
                return ExitCode::FAILURE
            } else {
                Config::default()
            }
        }
    };

    println!("{config:?}");

    let result = match args.cmd {
        Commands::Init    => init(),
        Commands::Regen   => regen(config),
        Commands::Empty   => empty(config),
        Commands::Exclude => exclude(config),
    };

    match result {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => ExitCode::from(e.code() as u8)
    }

}
