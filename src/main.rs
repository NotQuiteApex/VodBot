// VodBot (c) 2020-21 Logan "NotQuiteApex" Hickok-Dickson

use std::fs;
use std::path::Path;

extern crate clap;
extern crate dirs;
extern crate serde_json;
extern crate ansi_term;

use ansi_term::Color::{Red, Yellow};
use clap::{Arg, App, SubCommand};
use serde_json::Value;

#[macro_use]
mod util;
mod twitch;
mod commands {
	pub mod pull;
	pub mod stage;
	pub mod upload;
}

fn main() {
	// Load the environment variables from cargo for this info.
	const AUTHORS: Option<&'static str> = option_env!("CARGO_PKG_AUTHORS");
	const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

	// Get the .vodbot dir.
	// Error if the home directory does not exist.
	if dirs::home_dir().is_none() {
		panic!("Cannot find home directory.")
	}

	// Create base directory.
	let mut vodbot_dir = dirs::home_dir().unwrap(); vodbot_dir.push(".vodbot");
	match fs::create_dir_all(&vodbot_dir) {
		Err(why) => {panic!("Cannot create `.vodbot` in home directory. {}", why)},
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
		// Pull command, downloads VODs or Clips (or both).
		.subcommand(SubCommand::with_name("pull")
			.about("Pulls VODs or Clips from Twitch")
			.arg(Arg::with_name("type")
				.possible_values(&["vods", "clips", "both"])
				.help("Type of content to pull.")))
		// Stage command, for staging VODs/Clips for slicing/uploading.
		.subcommand(SubCommand::with_name("stage")
			.about("Stages VODs and Clips for slicing/uploading")
			// Adding a new stage.
			.subcommand(SubCommand::with_name("add")
				.about("Add a stage with a VOD or Clip ID.")
				.arg(Arg::with_name("id")
					.required(true)
					.help("ID of the VOD or Clip to stage.")))
			// Editing an existing stage.
			.subcommand(SubCommand::with_name("edit")
				.about("Edit an existing stage with its ID.")
				.arg(Arg::with_name("id")
					.required(true)
					.help("ID of the stage to edit.")))
			// Listing stages or getting details of a specific stage.
			.subcommand(SubCommand::with_name("list")
				.about("List current stages, or display info of a specific stage with an ID.")
				.arg(Arg::with_name("id")
					.help("ID of the stage to view. Optional.")))
			// Removing an existing stage.
			.subcommand(SubCommand::with_name("rm")
				.about("Remove an existing stage with an ID.")
				.arg(Arg::with_name("id")
					.required(true)
					.help("ID of the stage to remove."))))
		// Upload command, for uploading staged data to YouTube.
		.subcommand(SubCommand::with_name("upload")
			.about("Uploads stages to YouTube")
			.arg(Arg::with_name("id")
				.required(true)
				.help("ID of the stage to upload; \"all\" to upload all stages; \
						\"logout\" to logout of the YouTube account.")))
		// All done, parse input and get matches.
		.get_matches();
	
	// Load config.
	let mut default_config_path = vodbot_dir.clone();
	default_config_path.push("conf.json");

	let config_path = Path::new(
		matches.value_of("config")
		.unwrap_or(default_config_path.to_str().unwrap()));
	
	if !config_path.exists() {
		println!("Could not find config, attempting to create one...");
		util::create_conf(config_path);
	}

	let config: Value = util::load_conf(config_path);
	
	// Run the commands 
	let mut res: Result<(), util::ExitMsg> = Ok(());
	if let Some(matches) = matches.subcommand_matches("pull") {
		res = commands::pull::run(matches, config);
	} else if let Some(matches) = matches.subcommand_matches("stage") {
		res = commands::stage::run(matches, config);
	} else if let Some(matches) = matches.subcommand_matches("upload") {
		res = commands::upload::run(matches, config);
	}

	if res.is_err() {
		let err = res.unwrap_err();
		println!("{} {} {}",
			Red.bold().paint("Error! ("),
			Yellow.bold().paint(format!("{}", err.code as i32)),
			Red.bold().paint(format!(") {}", err.msg))
		);
		std::process::exit(err.code as i32);
	}
}
