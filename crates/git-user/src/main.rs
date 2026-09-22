use crate::args::Args;
use crate::config::Config;
use crate::error::AppError;
use crate::profile::Profile;
use clap::Parser;
use std::fs;
use std::path::Path;

mod args;
mod config;
mod error;
mod git;
mod profile;

fn main() {
    let args = Args::parse();
    if let Err(e) = handle_args(args) {
        println!("Error: {e}");
        std::process::exit(1);
    }
}

fn handle_args(args: Args) -> Result<(), AppError> {
    let mut config = get_or_create_config(&args.config)?;
    match args.command {
        args::Command::Add(add_args) => {
            config.insert(
                add_args.profile.as_deref().unwrap_or(&add_args.name),
                &Profile::new(add_args.name.clone(), add_args.email)
                    .with_signing_key(add_args.signing_key)
                    .with_ssh_command(add_args.ssh_command)
                    .with_gpg_format(add_args.gpg_format),
            );
            save_config(&args.config, &config)
        },
        args::Command::Delete { profile } => {
            config.remove(&profile);
            save_config(&args.config, &config)
        },
        args::Command::Export => {
            println!("{}", config.to_json()?);
            Ok(())
        },
        args::Command::List => {
            println!("{config}");
            Ok(())
        },
        args::Command::Use { profile, repo } => {
            let config = config
                .get(&profile)
                .ok_or_else(|| AppError::Config(format!("User '{profile}' not found.")))?;
            git::update_config(shellexpand::tilde(&repo).as_ref(), config)
        },
    }
}

fn save_config(path: &str, config: &Config) -> Result<(), AppError> {
    let expanded = shellexpand::tilde(path);
    let path = Path::new(expanded.as_ref());
    fs::write(path, config.to_json()?).map_err(|e| AppError::File(e.to_string()))
}

fn get_or_create_config(path: &str) -> Result<Config, AppError> {
    let expanded = shellexpand::tilde(path);
    let path = Path::new(expanded.as_ref());
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| AppError::File(e.to_string()))?;
    }

    if path.exists() {
        let content = fs::read_to_string(path).map_err(|e| AppError::File(e.to_string()))?;
        Config::from_json(content.as_str())
    } else {
        let result = Config::default();
        fs::write(path, result.to_json()?).map_err(|e| AppError::File(e.to_string()))?;
        Ok(result)
    }
}
