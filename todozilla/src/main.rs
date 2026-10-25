mod config;

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

fn init() {
    if Path::new("./TZ.yml").exists() {
        eprintln!("there's already a TZ.yml file")
    }
    todo!("init command")
}

fn regen(config: Config) {
    todo!("regen command")
}

fn empty(config: Config) {
    if config.ephemeral_heading.is_none() {
        eprintln!("the empty command requires Ephemeral TODOs to be enabled")
    }
    todo!("empty command")
}

fn exclude(config: Config) {
    if config.ephemeral_heading.is_none() {
        eprintln!("the exclude command requires Ephemeral TODOs to be enabled")
    }
    todo!("exclude command")
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

    match args.cmd {
        Commands::Init    => init(),
        Commands::Regen   => regen(config),
        Commands::Empty   => empty(config),
        Commands::Exclude => exclude(config),
    }

    ExitCode::SUCCESS
}
