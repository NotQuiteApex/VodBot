// Command to pull VODs and Clips from Twitch.

use super::super::twitch;
use super::super::util;


pub fn run(args: &clap::ArgMatches, mut config: serde_json::Value) -> Result<(), util::ExitMsg> {
	// Get content type.
	let pull_type = args.value_of("type").unwrap_or("both");
	
	// Create the web client.
	let client = reqwest::blocking::Client::new();

	// Pull necessary fields from the config
	load_key_config!(channel_ids, Vec<String>, config, "twitch_channels");
	load_key_config!(client_id, String, config, "twitch_client_id");
	load_key_config!(client_secret, String, config, "twitch_client_secret");

	// Get access_token from Twitch, used for using the APIs.
	let client_token = twitch::get_access_token(&client, &client_id, &client_secret);
	println!("{} {} {}", client_secret, client_secret, client_token);
	for name in channel_ids.iter() {
		println!("name: {}", name);
	}

	// All done, let's bounce
	Ok(())
}