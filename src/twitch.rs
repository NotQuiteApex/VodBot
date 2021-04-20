// Helper stuff for Twitch API

use super::util::{ExitCode, ExitMsg};

use reqwest::blocking::Client;
use serde::{Serialize, Deserialize};
use serde_json::{from_str, from_value, Value};

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
-> Result<String, ExitMsg> {

	let url = format!(
		"https://id.twitch.tv/oauth2/token?client_id={}\
		&client_secret={}&grant_type=client_credentials",
		client_id,
		client_secret,
	);

	match client.post(url).send() {
		Err(why) => Err( ExitMsg{ code: ExitCode::NoConnection,
			msg: format!("No response from Twitch for auth. Reason: \"{}\"", why)
		}),

		Ok(res) => match res.text() {
			Err(why) => Err( ExitMsg{ code: ExitCode::CannotParseResponse,
				msg: format!("Cannot read response from Twitch for auth. Reason: \"{}\"", why)
			}),

			Ok(response) => match from_str::<Value>(&response) {
				Err(why) => Err( ExitMsg{ code: ExitCode::CannotParseResponse,
					msg: format!("Cannot parse response from Twitch for auth. Reason: \"{}\"", why)
				}),

				Ok(mut parse) => match from_value::<String>(parse["access_token"].take()) {
					Err(why) => Err( ExitMsg{ code: ExitCode::CannotParseResponse,
						msg: format!("Cannot read key \"access_token\" from response from \
							Twitch for auth. Reason: \"{}\"", why)
					}),

					Ok(read) => Ok(read),
				},
			},
		},
	}
}


pub fn get_channels(channel_ids: &Vec<String>, cl: &Client, cl_id: &String, cl_tkn: &String)
-> Result<Vec<Channel>, ExitMsg> {

	if channel_ids.len() == 0 {
		return Err(ExitMsg{
			code: ExitCode::MissingConfigChannels,
			msg: String::from("Missing channel names in the config file.")
		});
	}

	let url = format!("https://api.twitch.tv/helix/users?login={}", channel_ids.join("&login="));

	let res = cl.get(url)
		.header("Client-ID", cl_id)
		.header("Authorization", format!("Bearer {}", cl_tkn))
		.send();
		
	return match res {
		Err(why) => Err( ExitMsg{ code: ExitCode::NoConnection,
			msg: format!("No response from Twitch for channels. Reason: \"{}\"", why)
		}),

		Ok(res) => match res.text() {
			Err(why) => Err( ExitMsg{ code: ExitCode::CannotParseResponse,
				msg: format!("Cannot read response from Twitch for channels. Reason: \"{}\"", why)
			}),

			Ok(response) => match from_str::<Value>(&response) {
				Err(why) => Err( ExitMsg{ code: ExitCode::CannotParseResponse,
					msg: format!("Cannot parse response from Twitch for channels. \
						Reason: \"{}\"", why)
				}),

				Ok(mut parse) => match from_value::<Vec<Channel>>(parse["data"].take()) {
					Err(why) => Err( ExitMsg{ code: ExitCode::CannotParseResponse,
						msg: format!("Cannot read key \"access_token\" from response from \
							Twitch for channels. Reason: \"{}\"", why)
					}),

					Ok(read) => Ok(read),
				},
			},
		},
	}
}


// pub fn get_channel_vods(channel: &Channel, cl: &Client, cl_id: &String, cl_tkn: &String)
// -> Result<Vec<VodData>, ExitMsg> {

// }
