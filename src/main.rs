extern crate clap;
extern crate dirs;
use clap::{Arg, App, SubCommand};

fn main() {
	// Load the environment variables from cargo for this info.
	const AUTHORS: Option<&'static str> = option_env!("CARGO_PKG_AUTHORS");
	const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

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
	
	let config_path = matches.value_of("config").unwrap_or("/default/location/conf.json");

	//println!("Hello, world! I'm VodBot!");
}
