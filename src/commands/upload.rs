// Command to upload videos to YouTube

use clap::ArgMatches;
use serde_json::Value;

pub fn run(_args: &ArgMatches, _config: Value) {
	// First check if the ID is allowed (is a stage ID, is all, is logout)
	println!("upload command initiate!")
}