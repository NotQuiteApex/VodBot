// Helper stuff for Twitch API

use reqwest;

struct Channel<'a> {
	id: &'a str,
	login: &'a str,
	display_name: &'a str,

	created_at: &'a str,
}

struct VodData<'a> {
	id: &'a str,
	title: &'a str,
	created_at: &'a str,
	duration: &'a str,

	streamer_id: &'a str,
	streamer_name: &'a str,
}

struct ClipData<'a> {
	id: &'a str,
	title: &'a str,
	created_at: &'a str,
	view_count: u64,

	streamer_id: &'a str,
	streamer_name: &'a str,

	clipper_id: &'a str,
	clipper_name: &'a str,
}

pub fn get_access_token(client_id: &str, client_secret: &str) {
	let URL = format!(
		"https://id.twitch.tv/oauth2/token?client_id={}&client_secret={}&grant_type=client_credentials",
		client_id,
		client_secret,
	);

	//let resp = reqwest::blocking::post();
}