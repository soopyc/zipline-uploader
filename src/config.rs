use std::{collections::HashMap, sync::OnceLock};

use crate::consts;
use clap::Args;
use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, Args)]
#[serde(rename_all = "kebab-case")]
pub struct ZiplineOptions {
	/// Zipline folder to upload to.
	///
	/// Provide the folder ID. For example, with https://zipline.example.moe/folder/cm3meow3colon3,
	/// the folder ID would be cm3meow3colon3.
	#[arg(long = "folder")]
	folder_id: Option<String>,

	/// Filename format presets to use.
	#[arg(short = 'F', long, value_enum)]
	filename_format: Option<consts::FilenameFormats>,

	/// Whether to save the original file name.
	///
	/// This is different from filename_format, in that this only affects the filename on download.
	#[arg(long)]
	save_original_name: Option<bool>,

	/// Image compression percentage. Might not do anything for your server.
	///
	/// Corresponds to the header x-zipline-image-compression-percent,
	/// takes in a range of 0 to 100.
	#[arg(long, value_parser = clap::value_parser!(u8).range(0..100))]
	compression_percentage: Option<u8>,

	/// A relative time until the file is deleted.
	///
	/// No validation is done, make sure the value is correct.
	///
	/// See https://zipline.diced.sh/docs/guides/ms for possible values.
	/// Conflicts with deletes-at.
	#[arg(short = 'T', long, conflicts_with = "deletes_at")]
	valid_for: Option<String>,

	/// A timestamp at which the file will be deleted.
	///
	/// See https://zipline.diced.sh/docs/guides/upload-options#javascript-resolvable-date-string
	/// for the format. Do not include `date=` as it is automatically added for you.
	///
	/// No validation is done for this value because JavaScript accepts a bajillion formats. Make
	/// sure you know that it works.
	/// Conflicts with valid-for.
	#[arg(long, conflicts_with = "valid_for")]
	deletes_at: Option<String>,

	/// A password that, when set, shall be entered by viewers to access the file(s).
	///
	/// Avoid passing as a command line option, either store it in the config file or use
	/// --prompt-password instead.
	#[arg(long)]
	password: Option<String>,
}

/*
 * Left unimplemented:
 * max-views
 * domain
 * (file-txtension?)
 */

/// main configuraton struct
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default, rename_all = "kebab-case")]
pub struct ConfigOption {
	remote_url: Url,
	token: String,

	/// A list of options for zipline, as headers.
	zipline_options: Option<ZiplineOptions>,

	/// A list of extra headers to use.
	extra_headers: Option<HashMap<String, String>>,
}

impl ConfigOption {
	pub fn new() -> Self {
		Self::default()
	}
}

impl Default for ConfigOption {
	fn default() -> Self {
		Self {
			remote_url: Url::parse("https://zipline.host.invalid").expect("bug: hardcoded url must be valid"),
			token: "AAAAAAAAAAAAAA==.PASTE/THE/TOKEN/HERE/FROM/ZIPLINE".into(),

			zipline_options: Some(ZiplineOptions::default()),
			extra_headers: None,
		}
	}
}

pub static CONFIG: OnceLock<ConfigOption> = OnceLock::new();
