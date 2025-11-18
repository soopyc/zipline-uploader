use std::{
	fs::{File, create_dir_all, read_to_string},
	io::Write,
};

use clap::Parser;
use color_eyre::Section;
use eyre::{Context, ContextCompat, Result};
use log::{debug, error, info, warn};

use crate::{
	config::{CONFIG, ConfigOption},
	consts::CONFIG_FILE,
};

mod api;
mod config;
mod consts;

/// A simple file uploader for Zipline (and compatible) hosts.
///
/// Compatible with Zipline API v4.
///
/// Source code: https://patchy.soopy.moe/soopyc/zipline-uploader
#[derive(Debug, Parser)]
#[command(version)]
struct Cli {
	/// A list of files to upload. Pass a directory to upload everything in said directory.
	#[arg(required = true, value_hint = clap::ValueHint::FilePath)]
	paths: Vec<String>,

	/// Don't actually do any uploading.
	#[arg(short = 'n', long)]
	dry_run: bool,

	#[command(flatten)]
	delegate: config::ZiplineOptions,

	/// Immediately quit on first failure.
	// Having to specify --keep-going on something like this is an anti-feature to me.
	#[arg(short = 'k', long)]
	fail_fast: bool,

	/// The filename for the upload
	///
	/// Only makes sense for a singular file. Will error if this is set but multiple files and/or
	/// a directory is passed in.
	// this doesn't make sense to put into zipline options.
	#[arg(long)]
	name: Option<String>,

	/// Prompt for a password to be set for the files.
	///
	/// This takes precedence over --password.
	#[arg(short = 'P', long)]
	prompt_password: bool,

	/// Do not confirm anything when uploading.
	#[arg(short, long)]
	yes: bool,
}

fn main() -> Result<()> {
	// initial setup
	color_eyre::install()?;
	colog::init();
	debug!("successfully completed pre-initialization.");

	// load or create config
	let config_file = match read_to_string(&*consts::CONFIG_FILE) {
		Ok(data) => data,
		Err(err) => match err.kind() {
			std::io::ErrorKind::NotFound => {
				debug!("config file doesn't exist when trying to read it, creating file.");
				create_dir_all(
					consts::CONFIG_FILE
						.parent()
						.wrap_err("while creating config dir tree")?,
				)?;
				let mut file = File::create_new(&*consts::CONFIG_FILE)
					.wrap_err("while trying to generate a config file")?;
				let buf = toml::to_string(&ConfigOption::new())?;
				file.write_all(buf.as_bytes())?;
				debug!(
					"wrote new configuration file at {}",
					consts::CONFIG_FILE.display()
				);

				info!(
					"A new configuration file has been generated at {}. Please edit it accordingly.",
					consts::CONFIG_FILE.display()
				);
				return Ok(());
			}
			_ => {
				return Err(err)
					.wrap_err("could not read config file")
					.with_note(|| format!("Config file: {}", consts::CONFIG_FILE.display()));
			}
		},
	};

	let cli = Cli::parse();

	match CONFIG.set(
		toml::from_str(&config_file)
			.wrap_err("while parsing the config")
			.with_suggestion(|| {
				format!(
					"try checking the config file `{}` for syntax errors",
					CONFIG_FILE.display()
				)
			})?,
	) {
		Ok(_) => (), // all good!
		Err(_) => {
			unreachable!("config oncelock was already set before the program was even initialized")
		}
	};

	println!("{:#?}", CONFIG.get().wrap_err("explode")?);

	Ok(())
}

mod tests {
	#[test]
	fn test_cli_clap() {
		use clap::CommandFactory;
		super::Cli::command().debug_assert();
	}
}
