// Command to upload videos to YouTube

use super::super::util;


pub fn run(args: &clap::ArgMatches, mut config: serde_json::Value) -> Result<(), util::ExitMsg> {
	// First check if the ID is allowed (is a stage ID, is all, is logout)
	println!("upload command initiate!");

	Ok(())
}