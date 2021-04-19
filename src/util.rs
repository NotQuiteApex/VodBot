// Utility Functions and Types

extern crate serde_json;

use std::path::Path;
use std::fs;
use std::io::BufReader;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone)]
pub enum ExitCode {
	_CleanExit,
	CannotCreateDir,
	NoConnection,
	CannotParseResponse,
	CannotFindAccessToken,
	MissingConfigChannels,
	CannotWriteConfig,
	WroteDefaultConfig,
	CannotParseConfig,
}

pub struct ExitMsg {
	pub msg: String,
	pub code: ExitCode,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
	pub twitch_channels: Vec<String>,
	
	pub twitch_client_id: String,
	pub twitch_client_secret: String,

	pub stage_timezone: String,
	pub stage_format: HashMap<String, String>,

	pub youtube_client_path: String,
	pub youtube_pickle_path: String,

	pub temp_dir: String,
	pub stage_dir: String,

	pub vods_dir: String,
	pub clips_dir: String,
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


pub fn create_conf(conf_path: &Path) -> Result<(), ExitMsg> {
	match fs::write(conf_path, DEFAULT_CONFIG) {
		Err(why) => Err(ExitMsg{
			code: ExitCode::CannotWriteConfig,
			msg: format!("Cannot write to config file at `{}`, reason: \"{}\".", conf_path.display(), why)
		}),
		Ok(_) => Err(ExitMsg{
			code: ExitCode::WroteDefaultConfig,
			msg: format!("Wrote default config file at `{}`, fill it out to continue.", conf_path.display())
		}),
	}?
}


pub fn load_conf(conf_path: &Path) -> Result<Config, ExitMsg> {
	let config_file = match fs::File::open(conf_path) {
		Err(why) => panic!("Cannot open config file. {}", why),
		Ok(file) => file
	};

	let reader = BufReader::new(config_file);

	let json: serde_json::Result<Config> = serde_json::from_reader(reader);
	if let Ok(config) = json {
		Ok(config)
	} else {
		Err(ExitMsg{
			code: ExitCode::CannotParseConfig,
			msg: String::from("Cannot parse config.")
		})
	}

	// TODO: Do some checks for values.
}


pub fn create_dir(dir_path: &Path) -> Result<(), ExitMsg> {
	match fs::create_dir_all(&dir_path) {
		Err(why) => {
			Err(ExitMsg{
				code: ExitCode::CannotCreateDir,
				msg: format!("Cannot create directory `{}`, reason: \"{}\".", 
					&dir_path.display(), why)
			})
		},
		_ => Ok(())
	}
}
