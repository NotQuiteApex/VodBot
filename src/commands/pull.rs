// Command to pull VODs and Clips from Twitch.

use std::path::PathBuf;

use super::super::twitch;
use super::super::util;


enum VideoType {
	Vod(twitch::VodData),
	Clip(twitch::ClipData),
}


pub fn run(args: &clap::ArgMatches, config: util::Config) -> Result<(), util::ExitMsg> {
	// Get content type.
	let pull_type = args.value_of("type").unwrap_or("both").to_owned();
	let get_both = pull_type == "both";
	let get_vods = get_both || pull_type == "vods";
	let get_clips = get_both || pull_type == "clips";
	
	// Create the web client.
	let client = reqwest::blocking::Client::new();

	// Pull necessary fields from the config
	let channel_ids = config.twitch_channels;
	let client_id = config.twitch_client_id;
	let client_secret = config.twitch_client_secret;
	let temp_dir = PathBuf::from(&config.temp_dir);
	let vods_dir = PathBuf::from(&config.vods_dir);
	let clips_dir = PathBuf::from(&config.clips_dir);

	// Create the necessary directories.
	util::create_dir(&temp_dir)?;
	util::create_dir(&vods_dir)?;
	util::create_dir(&clips_dir)?;

	// Get access_token from Twitch, used for using the APIs.
	let client_token = twitch::get_access_token(&client, &client_id, &client_secret)?;

	// Get the channel data from Twitch.
	let channels = twitch::get_channels(&channel_ids, &client, &client_id, &client_token)?;

	// A place to store videos.
	let videos: Vec<VideoType> = Vec::new();

	// Start pulling content info of each channel.
	for channel in channels.iter() {
		// Grab VOD info
		if get_vods {
			// Create VOD directory for channel.
			let mut channel_dir = vods_dir.clone();
			channel_dir.push(&channel.login);
			util::create_dir(&channel_dir)?;
			// Pull VODs from Twitch.
			
		}

		// Grab Clip info
		if get_clips {
			// Create Clip directory for channel.
			let mut channel_dir = vods_dir.clone();
			channel_dir.push(&channel.login);
			util::create_dir(&channel_dir)?;

		}

		// Print results
		if get_both {

		} else if get_vods {

		} else if get_clips {

		}
	}

	println!("pull-type: {}", pull_type);
	println!("temp:  {}", temp_dir.display());
	println!("vods:  {}", vods_dir.display());
	println!("clips: {}", clips_dir.display());
	println!();
	print!("id: {} | ", client_id);
	print!("secret: {} | ", client_secret);
	print!("token: {}", client_token);
	println!(); println!();
	print!("channels: ");
	for name in channel_ids.iter() {
		print!("{} ", name);
	}
	println!();
	print!("found: ");
	for channel in channels.iter() {
		print!("{} ", channel.login);
	}
	println!();

	// All done, let's bounce
	Ok(())
}