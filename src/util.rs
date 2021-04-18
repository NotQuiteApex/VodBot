// Utility Functions and Types

extern crate serde_json;

use std::path;
use std::fs;
use std::io::BufReader;

#[derive(Copy, Clone)]
pub enum ExitCode {
	_CleanExit,
	MissingFromConfig,
}

pub struct ExitMsg {
	pub msg: &'static str,
	pub code: ExitCode,
}


macro_rules! load_key_config {
	($var:ident, $type:ty, $conf:ident, $key:expr) => {
		let $var: $type;
		if let Ok(j) = serde_json::from_value($conf[$key].take()) {
			$var = j;
		} else {
			return Err( util::ExitMsg{
				code: util::ExitCode::MissingFromConfig,
				msg: concat!("Cannot load key ", stringify!($key), " from config.")
			});
		}
	};
}


const DEFAULT_CONFIG: &str = r#"
{
	"twitch_channels": [ "46moura",
		"alkana", "batkigu", "hylianswordsman1"
		"juicibit", "michiri9", "notquiteapex",
		"pissyellowcrocs", "percy_creates", "voobo",
	],
	
	"twitch_client_id": "[[INSERT CLIENT ID HERE]]",
	"twitch_client_secret": "[[INSERT CLIENT SECRET HERE]]",
	
	"stage_timezone": "US/Eastern",
	"stage_format": {
		"watch": "-- Watch live at {links}",
		"discord": "-- Join the Discord https://discord.gg/v2t6uag",
		"credits": "\n{watch}\n{discord}"
	},
	
	"youtube_client_path": "{vodbot}/yt-client.json",
	"youtube_pickle_path": "{vodbot}/yt-pickle.pikl",

	"vod_dir": "{vodbot}/vods",
	"clip_dir": "{vodbot}/clips",
	"temp_dir": "{vodbot}/temp",
	"stage_dir": "{vodbot}/stage",
}
"#;


pub fn create_conf(conf_path: &path::Path) {
	fs::write(conf_path, DEFAULT_CONFIG)
		.expect(format!("Cannot write to config file at {}.", conf_path.display()).as_str());
	
	panic!("Created config at {}, please edit it to continue. Exiting...", conf_path.display())
}


pub fn load_conf(conf_path: &path::Path) -> serde_json::Value {
	let config_file = match fs::File::open(conf_path) {
		Err(why) => panic!("Cannot open config file. {}", why),
		Ok(file) => file
	};
	let reader = BufReader::new(config_file);
	let conf: serde_json::Value = serde_json::from_reader(reader).expect("Could not parse config.");

	// TODO: Do some checks for values.

	conf
}
