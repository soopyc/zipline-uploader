use std::{path::PathBuf, sync::LazyLock};

use clap::ValueEnum;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

pub static PROJECT_PATH: LazyLock<ProjectDirs> =
	LazyLock::new(|| ProjectDirs::from("moe", "soopy", "zipline-uploader").unwrap());

pub static CONFIG_FILE: LazyLock<PathBuf> =
	LazyLock::new(|| PROJECT_PATH.config_dir().join("config.toml"));

#[derive(Clone, Copy, ValueEnum, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", deny_unknown_fields)]
pub enum FilenameFormats {
	/// left unset, will use whatever the server defaults to or x-zipline-filename header if set.
	Unset,
	/// random value according to server settings
	Random,
	/// date format according to server settings
	Date,
	/// a random uuidv4
	Uuid,
	/// the original filename
	Name,
	/// random words, see https://zipline.diced.sh/docs/guides/upload-options#random-words
	Gfycat,
}

impl Default for FilenameFormats {
	fn default() -> Self {
		Self::Unset
	}
}
