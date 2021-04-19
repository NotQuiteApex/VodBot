// Helper stuff for Twitch API

use super::util;

use reqwest::blocking::Client;


struct Channel {
	id: String,
	login: String,
	display_name: String,

	created_at: String,
}

struct VodData {
	id: String,
	title: String,
	created_at: String,
	duration: String,

	streamer_id: String,
	streamer_name: String,
}

struct ClipData {
	id: String,
	title: String,
	created_at: String,
	view_count: u64,

	streamer_id: String,
	streamer_name: String,

	clipper_id: String,
	clipper_name: String,
}


pub fn get_access_token(client: &Client, client_id: &String, client_secret: &String) -> Result<String, util::ExitMsg> {
	let url = format!(
		"https://id.twitch.tv/oauth2/token?client_id={}&client_secret={}&grant_type=client_credentials",
		client_id,
		client_secret,
	);

	let res = client.post(url).send();

	if let Ok(response) = res {
		if let Ok(text) = response.text() {
			let parse: serde_json::Result<serde_json::Value> = serde_json::from_str(&text);
			if let Ok(mut json) = parse {
				let read: serde_json::Result<String> = serde_json::from_value(json["access_token"].take());
				if let Ok(token) = read {
					Ok(token)
				} else {
					Err(util::ExitMsg{
						code: util::ExitCode::CannotFindAccessToken,
						msg: String::from("Cannot read key \"access_token\" from response from Twitch for auth.")
					})
				}
			} else {
				Err(util::ExitMsg{
					code: util::ExitCode::CannotParseResponse,
					msg: String::from("Cannot parse response as JSON from Twitch for auth.")
				})
			}
		} else {
			Err(util::ExitMsg{
				code: util::ExitCode::CannotParseResponse,
				msg: String::from("Cannot read response from Twitch for auth.")
			})
		}
	} else {
		Err(util::ExitMsg{
			code: util::ExitCode::NoConnection,
			msg: String::from("No response from Twitch for auth.")
		})
	}
}