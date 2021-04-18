// Command to pull VODs and Clips from Twitch.

use super::super::twitch;

use clap::ArgMatches;
use serde_json::Value;

pub fn run(args: &ArgMatches, mut config: Value) {
	let pull_type = args.value_of("type").unwrap_or("both");
	println!("pull {} command go!", pull_type);

	let client_id: String = serde_json::from_value(config["twitch_client_id"].take())
		.expect("Cannot read key \"twitch_client_id\" from config.");
	let client_secret: String = serde_json::from_value(config["twitch_client_secret"].take())
		.expect("Cannot read key \"twitch_client_secret\" from config.");

	let token = twitch::get_access_token(client_id.as_str(), client_secret.as_str());
	println!("{} {}", client_secret, client_secret);
}