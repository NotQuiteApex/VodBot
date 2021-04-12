// Command to pull VODs and Clips from Twitch.

use clap::ArgMatches;
use serde_json::Value;

pub fn run(args: &ArgMatches, config: Value) {
	let pull_type = args.value_of("type").unwrap_or("both");
	println!("{}", pull_type);
	println!("pull command go!")
}