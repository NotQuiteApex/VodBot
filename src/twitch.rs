// Helper stuff for Twitch API

use super::util;

use reqwest::blocking::Client;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Channel {
	pub id: String,
	pub login: String,
	pub display_name: String,

	pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct VodData {
	pub id: String,
	pub title: String,
	pub created_at: String,
	pub duration: String,

	pub streamer_id: String,
	pub streamer_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ClipData {
	pub id: String,
	pub title: String,
	pub created_at: String,
	pub view_count: u64,

	pub streamer_id: String,
	pub streamer_name: String,

	pub clipper_id: String,
	pub clipper_name: String,
}


pub fn get_access_token(client: &Client, client_id: &String, client_secret: &String)
-> Result<String, util::ExitMsg> {

	let url = format!(
		"https://id.twitch.tv/oauth2/token?client_id={}\
		&client_secret={}&grant_type=client_credentials",
		client_id,
		client_secret,
	);

	let res = client.post(url).send();

	if let Ok(response) = res {
		if let Ok(text) = response.text() {
			let parse: serde_json::Result<serde_json::Value> = serde_json::from_str(&text);
			if let Ok(mut json) = parse {
				let read: serde_json::Result<String> = serde_json::from_value(
					json["access_token"].take());
				if let Ok(token) = read {
					Ok(token)
				} else {
					Err(util::ExitMsg{
						code: util::ExitCode::CannotFindAccessToken,
						msg: String::from("Cannot read key \"access_token\" \
							from response from Twitch for auth.")
					})
				}
			} else {
				Err(util::ExitMsg{
					code: util::ExitCode::CannotParseResponse,
					msg: String::from("Cannot parse response from Twitch for auth.")
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


pub fn get_channels(channel_ids: &Vec<String>, cl: &Client, cl_id: &String, cl_tkn: &String)
-> Result<Vec<Channel>, util::ExitMsg> {

	if channel_ids.len() == 0 {
		return Err(util::ExitMsg{
			code: util::ExitCode::MissingConfigChannels,
			msg: String::from("Missing channel names in the config file.")
		});
	}

	let url = format!("https://api.twitch.tv/helix/users?login={}", channel_ids.join("&login="));

	let res = cl.get(url)
		.header("Client-ID", cl_id)
		.header("Authorization", format!("Bearer {}", cl_tkn))
		.send();
		
	if let Ok(response) = res {
		if let Ok(text) = response.text() {
			let parse: serde_json::Result<serde_json::Value> = serde_json::from_str(&text);
			if let Ok(mut json) = parse {
				let read: serde_json::Result<Vec<Channel>> = serde_json::from_value(
					json["data"].take());
				if let Ok(channels) = read {
					Ok(channels)
				} else {
					Err(util::ExitMsg{
						code: util::ExitCode::CannotParseResponse,
						msg: String::from("Cannot pull channels from Twitch response.")
					})
				}
			} else {
				Err(util::ExitMsg{
					code: util::ExitCode::CannotParseResponse,
					msg: String::from("Cannot parse response from Twitch for channels.")
				})
			}
		} else {
			Err(util::ExitMsg{
				code: util::ExitCode::CannotParseResponse,
				msg: String::from("Cannot read response from Twitch for channels.")
			})
		}
	} else {
		Err(util::ExitMsg{
			code: util::ExitCode::NoConnection,
			msg: String::from("No response from Twitch for channels.")
		})
	}
}
