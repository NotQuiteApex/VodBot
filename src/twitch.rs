// Helper stuff for Twitch API

use std::collections::HashMap;

use super::util::{ExitCode, ExitMsg};

use reqwest::blocking::Client;
use serde::{Serialize, Deserialize};
use serde_json::{from_str, from_value, Value};

#[derive(Serialize, Deserialize)]
pub struct Channel {
	pub id: String,
	pub login: String,
	pub display_name: String,

	pub broadcaster_type: String,
	pub description: String,
	pub view_count: u64,

	pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct VodData {
	// Metadata
	pub id: String,
	pub title: String,
	pub created_at: String,
	pub duration: String,
	pub view_count: u64,
	pub thumbnail_url: String,

	// Streamer
	pub user_id: String,
	pub user_name: String,
	pub user_login: String,
}

#[derive(Serialize, Deserialize)]
pub struct ClipData {
	// Metadata
	pub id: String,
	pub title: String,
	pub created_at: String,
	pub duration: f32,
	pub view_count: u64,
	pub thumbnail_url: String,

	// Streamer
	pub broadcaster_id: String,
	pub broadcaster_name: String,

	// Clipper
	pub creator_id: String,
	pub creator_name: String,
}

pub enum VideoType {
	Vod(VodData),
	Clip(ClipData),
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
						msg: format!("Cannot read key \"data\" from response from \
							Twitch for channels. Reason: \"{}\"", why)
					}),

					Ok(read) => Ok(read),
				},
			},
		},
	}
}


pub fn get_channel_vods(channel: &Channel, cl: &Client, cl_id: &String, cl_tkn: &String)
-> Result<Vec<VodData>, ExitMsg> {
	let base_url = format!(
		"https://api.twitch.tv/helix/videos?user_id={}&first=100&type=archive",
		channel.id
	);

	let mut vods: Vec<VodData> = Vec::new();

	let mut pagination: Option<String> = None;
	loop {
		// Create the URL
		let mut url = base_url.clone();
		if let Some(page) = pagination {
			url = url + "&after=" + page.as_str();
		}

		// Use it, REST API style
		let res = cl.get(url)
			.header("Client-ID", cl_id)
			.header("Authorization", format!("Bearer {}", cl_tkn));

		// Handle the response.
		let mut parse = match res.send() {
			Err(why) => Err( ExitMsg{ code: ExitCode::NoConnection,
				msg: format!("No response from Twitch for VODs. Reason: \"{}\"", why)
			}),
			Ok(res) => match res.text() {
				Err(why) => Err( ExitMsg{ code: ExitCode::CannotParseResponse,
					msg: format!("Cannot read response from Twitch for VODs. Reason: \"{}\"", why)
				}),
	
				Ok(response) => match from_str::<Value>(&response) {
					Err(why) => Err( ExitMsg{ code: ExitCode::CannotParseResponse,
						msg: format!("Cannot parse response from Twitch for VODs. \
							Reason: \"{}\"", why)
					}),
	
					Ok(parse) => Ok(parse)
				}
			}
		}?;

		// Pull the data into a list
		if let Ok(mut new_vods) = from_value::<Vec<VodData>>(parse["data"].take()) {
			if new_vods.len() == 0 {
				break;
			}
			for _ in 0..new_vods.len() {
				let vod = new_vods.pop().unwrap();
				if !vod.thumbnail_url.is_empty() {
					vods.push(vod);
				}
			}
		} else {
			break;
		}

		// Handle pagination
		if let Ok(page) = from_value::<HashMap<String, String>>(parse["pagination"].take()) {
			pagination = Some(page["cursor"].clone());
		} else {
			break;
		}
	}

	Ok(vods)
}


pub fn get_channel_clips(channel: &Channel, cl: &Client, cl_id: &String, cl_tkn: &String)
-> Result<Vec<ClipData>, ExitMsg> {
	let base_url = format!(
		"https://api.twitch.tv/helix/clips?broadcaster_id={}&first=100",
		channel.id
	);

	let mut clips: Vec<ClipData> = Vec::new();

	let mut pagination: Option<String> = None;
	loop {
		// Create the URL
		let mut url = base_url.clone();
		if let Some(page) = pagination {
			url = url + "&after=" + page.as_str();
		}

		// Use it, REST API style
		let res = cl.get(url)
			.header("Client-ID", cl_id)
			.header("Authorization", format!("Bearer {}", cl_tkn));

		// Handle the response.
		let mut parse = match res.send() {
			Err(why) => Err( ExitMsg{ code: ExitCode::NoConnection,
				msg: format!("No response from Twitch for Clips. Reason: \"{}\"", why)
			}),
			Ok(res) => match res.text() {
				Err(why) => Err( ExitMsg{ code: ExitCode::CannotParseResponse,
					msg: format!("Cannot read response from Twitch for Clips. Reason: \"{}\"", why)
				}),
	
				Ok(response) => match from_str::<Value>(&response) {
					Err(why) => Err( ExitMsg{ code: ExitCode::CannotParseResponse,
						msg: format!("Cannot parse response from Twitch for Clips. \
							Reason: \"{}\"", why)
					}),
	
					Ok(parse) => Ok(parse)
				}
			}
		}?;

		// Pull the data into a list
		if let Ok(mut new_clips) = from_value::<Vec<ClipData>>(parse["data"].take()) {
			if new_clips.len() == 0 {
				break;
			}
			for _ in 0..new_clips.len() {
				let vod = new_clips.pop().unwrap();
				if !vod.thumbnail_url.is_empty() {
					clips.push(vod);
				}
			}
		} else {
			break;
		}

		// Handle pagination
		if let Ok(page) = from_value::<HashMap<String, String>>(parse["pagination"].take()) {
			if page.contains_key("cursor") {
				pagination = Some(page["cursor"].clone());
			} else {
				break;
			}
		} else {
			break;
		}
	}

	Ok(clips)
}
