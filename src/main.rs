#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]

use std::{
	env::{self, VarError},
	ffi::OsStr,
	path::PathBuf,
};

fn main() {
	// First, get the value of the CARGO_HOME environment variable. This is one
	// of the few (two?) CARGO_* variables that is set both when running via
	// `cargo run` as well as via `cargo` itself as a subcommand.
	let cargo_home: PathBuf = match env::var("CARGO_HOME") {
		Ok(cargo_home) => cargo_home,
		Err(VarError::NotPresent) => {
			eprintln!("CARGO_HOME not set; unable to search registry cache.");
			return;
		}
		Err(e) => {
			eprintln!("Failed to get environment variable CARGO_HOME: {:?}", e);
			return;
		}
	}
	.into();

	if !cargo_home.as_path().is_dir() {
		eprintln!(
			"CARGO_HOME is not a directory: {:?}",
			cargo_home.as_os_str()
		);
	}

	let cache_glob = {
		let mut glob = cargo_home;
		glob.push("registry");
		glob.push("cache");
		glob.push("github.com-*");
		glob.push("*.crate");
		glob
	};

	let cache_glob: &str = cache_glob
		.as_os_str()
		.to_str()
		.expect("failed to add glob components to weird cache directory");

	for entry in glob::glob(cache_glob).expect("glob is weird") {
		match entry {
			Ok(path) => println!(
				"{}",
				path
					.file_name()
					.and_then(OsStr::to_str)
					.expect("failed to get file name for file")
			),
			Err(error) => eprintln!("{:?}", error),
		}
	}
}
