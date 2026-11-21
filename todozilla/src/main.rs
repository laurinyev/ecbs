mod config;
mod error;
mod marking;
mod tui;

use std::{io::Write, path::Path, process::ExitCode};
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
    
    let eph_enabled = match tui::prompt_yn("Set up ephemeral TODOs?") {
        Ok(v) => v,
        Err(e) => return error::error(error::ErrorCode::INTERNAL, Some(e.to_string())) 
    };

    let commit_hook = match Path::new(".git").exists() { 
        true => match tui::prompt_yn("Set up a git commit hook?") {
            Ok(v) => v,
            Err(e) => return error::error(error::ErrorCode::INTERNAL, Some(e.to_string())) 
        },
        false => false
    };

    let markings_heading = match tui::prompt_str_default("Name for markings section?", "Code Markings") {
        Ok(v) => match v.as_str() {
            "Code Markings" => None,
            _ => Some(v)
        },
        Err(e) => return error::error(error::ErrorCode::INTERNAL, Some(e.to_string())) 
    };

    let ephemeral_heading = match eph_enabled {
        true => match tui::prompt_str_default("Name for ephemeral section?", "Ephemeral TODOs") {
            Ok(v) => Some(v),
            Err(e) => return error::error(error::ErrorCode::INTERNAL, Some(e.to_string())) 
        },
        false => None
    };

    let config = Config::new(&markings_heading, &ephemeral_heading);
    match config.write(&Path::new("./TZ.yml")) {
        Ok(_) => (),
        Err(e) => return error::error(error::ErrorCode::INTERNAL, Some(e.to_string())) 
    };

    if commit_hook {
        match Path::new(".git/hooks/pre-commit").exists() {
            true => {
                let mut file = match std::fs::OpenOptions::new().create(true).append(true).open(Path::new(".git/hooks/pre-commit")) {
                    Ok(v) => v,
                    Err(e) => return error::error(error::ErrorCode::INTERNAL, Some(e.to_string())) 
                };

                match file.write_all(b"\ntodozilla regen\ntodozilla exclude") {
                    Ok(_) => (),
                    Err(e) => return error::error(error::ErrorCode::INTERNAL, Some(e.to_string())) 
                };
            },
            false => {
                let mut file = match std::fs::File::create(Path::new(".git/hooks/pre-commit")) {
                    Ok(v) => v,
                    Err(e) => return error::error(error::ErrorCode::INTERNAL, Some(e.to_string())) 
                };

                match file.write_all(b"#!/bin/sh\ntodozilla regen\ntodozilla exclude") {
                    Ok(_) => (),
                    Err(e) => return error::error(error::ErrorCode::INTERNAL, Some(e.to_string())) 
                };
            }
        }
    }

    let mut file = match std::fs::File::create(Path::new("TZ.md")) {
        Ok(v) => v,
        Err(e) => return error::error(error::ErrorCode::INTERNAL, Some(e.to_string())) 
    };

    let markings_write = match markings_heading {
        Some(v) => format!("# {v}\n\n"),
        None => format!("# Code Markings\n\n")    
    };

    match file.write_all(markings_write.as_bytes()) {
        Ok(_) => (),
        Err(e) => return error::error(error::ErrorCode::INTERNAL, Some(e.to_string())) 
    };

    if let Some(v) = ephemeral_heading {
        match file.write_all(format!("# {v}\n\n").as_bytes()) {
            Ok(_) => (),
            Err(e) => return error::error(error::ErrorCode::INTERNAL, Some(e.to_string())) 
        };       
    }

    Ok(())
}

fn regen(config: Config) -> error::Result<()> {
    let _ = marking::discover_markings(config);

    Ok(())
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
    let config = match config::Config::open(&Path::new("./TZ.yml")) {
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
