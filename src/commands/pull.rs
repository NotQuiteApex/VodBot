// Command to pull VODs and Clips from Twitch.

use std::path::PathBuf;

use super::super::twitch;
use super::super::util;

use ansi_term::Color::{Purple, Cyan, White};


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
	print!("{}", White.dimmed().paint("Logging in to Twitch.tv... "));
	let client_token = twitch::get_access_token(&client, &client_id, &client_secret)?;

	// Get the channel data from Twitch.
	print!("{}", White.dimmed().paint("Getting User ID's... "));
	let channels = twitch::get_channels(&channel_ids, &client, &client_id, &client_token)?;

	println!();

	// A place to store videos.
	let mut videos: Vec<twitch::VideoType> = Vec::new();
	// Info counts
	let mut total_vod_count: usize = 0;
	let mut total_clip_count: usize = 0;

	// Start pulling content info of each channel.
	for channel in channels.iter() {
		// Info counts
		let mut vod_count: usize = 0;
		let mut clip_count: usize = 0;

		// Print results
		if get_both {
			println!("Pulling {} & {} list for {}",
				Purple.bold().paint("VOD"), Purple.bold().paint("Clip"), channel.display_name);
		} else if get_vods {
			println!("Pulling {} list for {}", Purple.bold().paint("VOD"), channel.display_name);
		} else if get_clips {
			println!("Pulling {} list for {}", Purple.bold().paint("Clip"), channel.display_name);
		}

		// Grab VOD info
		if get_vods {
			// Create VOD directory for channel.
			let mut channel_dir = vods_dir.clone();
			channel_dir.push(&channel.login);
			util::create_dir(&channel_dir)?;
			// Pull VOD data from Twitch.
			let mut vods = twitch::get_channel_vods(channel, &client, &client_id, &client_token)?;
			vod_count = vods.len();
			for _ in 0..vods.len() {
				let vod = vods.pop().unwrap();
				if vod.thumbnail_url.is_empty() {
					videos.push(twitch::VideoType::Vod(vod));
				}
			}
		}

		// Grab Clip info
		if get_clips {
			// Create Clip directory for channel.
			let mut channel_dir = vods_dir.clone();
			channel_dir.push(&channel.login);
			util::create_dir(&channel_dir)?;
			// Pull Clip data from Twitch.
			let mut clips = twitch::get_channel_clips(channel, &client, &client_id, &client_token)?;
			clip_count = clips.len();
			for _ in 0..clips.len() {
				let clip = clips.pop().unwrap();
				if clip.thumbnail_url.is_empty() {
					videos.push(twitch::VideoType::Clip(clip));
				}
			}
		}

		// Print results
		if get_vods {
			println!("{} to download: {}",
				Purple.bold().paint("VODs"),
				Cyan.bold().paint(format!("{}", vod_count))
			);
			total_vod_count += vod_count;
		}
		if get_clips {
			println!("{} to download: {}",
				Purple.bold().paint("Clips"),
				Cyan.bold().paint(format!("{}", clip_count))
			);
			total_clip_count += clip_count;
		}
		if get_both {
			println!("{} to download: {}",
				Purple.bold().paint("Videos"),
				Cyan.bold().paint(format!("{}", vod_count + clip_count))
			);
		}
		println!();
	}

	// Print total results
	if get_vods {
		println!("Total {} to download: {}",
			Purple.bold().paint("VODs"),
			Cyan.bold().paint(format!("{}", total_vod_count))
		);
	}
	if get_clips {
		println!("Total {} to download: {}",
			Purple.bold().paint("VODs"),
			Cyan.bold().paint(format!("{}", total_clip_count))
		);
	}
	if get_both {
		println!("Total {} to download: {}",
			Purple.bold().paint("Videos"),
			Cyan.bold().paint(format!("{}", total_vod_count + total_clip_count))
		);
	}
	println!();


	// All done, let's bounce
	Ok(())
}