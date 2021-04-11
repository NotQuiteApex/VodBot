// VodBot (c) 2020-21 Logan "NotQuiteApex" Hickok-Dickson

use std::fs;

extern crate clap;
extern crate dirs;
use clap::{Arg, App, SubCommand};

mod util;

fn main() {
	// Load the environment variables from cargo for this info.
	const AUTHORS: Option<&'static str> = option_env!("CARGO_PKG_AUTHORS");
	const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

	// Get the .vodbot dir.
	// Error if the home directory does not exist.
	if dirs::home_dir().is_none() {
		util::exit_prog(util::ExitStat::MissingHomeDirectory, Some("Cannot find home directory."));
	}

	// Create base directory.
	let mut vodbot_dir = dirs::home_dir().unwrap(); vodbot_dir.push(".vodbot");
	match fs::create_dir_all(&vodbot_dir) {
		Err(_error) => {
			util::exit_prog(util::ExitStat::MissingHomeDirectory, Some("Cannot create `.vodbot` directory."));
		},
		_ => ()
	}

	// Run the argument parser.
	let matches = App::new("VodBot")
		.version(VERSION.unwrap_or("VERSION UNKNOWN"))
		.author(AUTHORS.unwrap_or("AUTHORS UNKNOWN"))
		.about("A VOD and Clip Manager for Twitch.")
		.arg(Arg::with_name("config")
			.short("c")
			.long("config")
			.value_name("FILE")
			.help("Sets the location of the config file.")
			.takes_value(true))

		.subcommand(SubCommand::with_name("pull")
			.about("Pulls VODs or Clips from Twitch")
			.arg(Arg::with_name("type")
				.help("Type of content to pull, such as VODs, Clips, or both.")
				))

		.subcommand(SubCommand::with_name("stage")
			.about("Stages VODs and Clips for slicing/uploading"))

		.subcommand(SubCommand::with_name("upload")
			.about("Uploads stages to YouTube"))

		.get_matches();
	
	let mut default_config_path = vodbot_dir.clone(); default_config_path.push("conf.json");
	let config_path = matches.value_of("config").unwrap_or(default_config_path.to_str().unwrap());

	if let Some(matches) = matches.subcommand_matches("pull") {
		println!("pull command go!")
	} else if let Some(matches) = matches.subcommand_matches("stage") {
		println!("stage command active!")
	} else if let Some(matches) = matches.subcommand_matches("upload") {
		println!("upload command initiate!")
	}
}
