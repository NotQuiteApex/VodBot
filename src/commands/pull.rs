// Command to pull VODs and Clips from Twitch.

use super::super::twitch;

pub fn run(args: &clap::ArgMatches, mut config: serde_json::Value) {
	// Get content type.
	let pull_type = args.value_of("type").unwrap_or("both");
	
	// Create the web client.
	let client = reqwest::blocking::Client::new();

	// Pull necessary fields from the config
	let client_id: String = serde_json::from_value(config["twitch_client_id"].take())
		.expect("Cannot read key \"twitch_client_id\" from config.");
	let client_secret: String = serde_json::from_value(config["twitch_client_secret"].take())
		.expect("Cannot read key \"twitch_client_secret\" from config.");

	// Get access_token from Twitch, used for using the APIs.
	let client_token = twitch::get_access_token(&client, &client_id, &client_secret);
	println!("{} {} {}", client_secret, client_secret, client_token);
}