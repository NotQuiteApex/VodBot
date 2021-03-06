from vodbot import util, twitch
from vodbot.itd import download as itd_dl, worker as itd_work
from vodbot.twitch import Channel, Vod, Clip
from vodbot.printer import cprint, colorize

from pathlib import Path
from os import listdir
from os.path import isfile


# Default path
vodbotdir = util.vodbotdir


def run(args):
	download_twitch_video(args)


def download_twitch_video(args):
	# Load the config and set up the access token
	cprint("#r#dLoading config...", end=" ", flush=True)
	conf = util.load_conf(args.config)
	CHANNEL_IDS = conf["twitch_channels"]
	CLIENT_ID = conf["twitch_client_id"]
	CLIENT_SECRET = conf["twitch_client_secret"]
	VODS_DIR = conf["vod_dir"]
	CLIPS_DIR = conf["clip_dir"]

	cprint("Logging in to Twitch.tv...", end=" ", flush=True)
	HEADERS = twitch.get_access_token(CLIENT_ID, CLIENT_SECRET)

	# If command line has channels to watch instead, use those instead of the config ones.
	if len(args.channels) != 0:
		CHANNELS = args.channels
	
	contentnoun = "video" # temp contentnoun until the rest is reworked
	
	# Setup directories for videos, config, and temp
	util.make_dir(vodbotdir)
	util.make_dir(vodbotdir / "temp")
	voddir = Path(VODS_DIR)
	util.make_dir(voddir)
	clipdir = Path(CLIPS_DIR)
	util.make_dir(clipdir)

	# Get user channel objects from Twitch API
	cprint("Getting User ID's...#r", flush=True)
	channels = twitch.get_channels(CHANNEL_IDS, HEADERS)

	# Get list of videos using channel object ID's from Twitch API
	videos = []
	totalvods = 0
	totalclips = 0

	channel_print = None
	if args.type == "both":
		channel_print = "Pulling #fM#lVOD#r & #fM#lClip#r list: #fY#l{}#r..."
	elif args.type == "vods":
		channel_print = "Pulling #fM#lVOD#r list: #fY#l{}#r..."
	elif args.type == "clips":
		channel_print = "Pulling #fM#lClip#r list: #fY#l{}#r..."

	for channel in channels:
		vods = None
		clips = None
		cprint(channel_print.format(channel.display_name), end=" ")

		# Grab list of VODs and check against existing VODs
		if args.type == "both" or args.type == "vods":
			folder = voddir / channel.login
			util.make_dir(folder)
			allvods = twitch.get_channel_vods(channel, HEADERS)
			vods = compare_existant_file(folder, allvods)
			totalvods += len(vods)

		# Grab list of Clips and check against existing Clips
		if args.type == "both" or args.type == "clips":
			folder = clipdir / channel.login
			util.make_dir(folder)
			allclips = twitch.get_channel_clips(channel, HEADERS)
			clips = compare_existant_file(folder, allclips)
			totalclips += len(clips)

		# Print content found and save it
		if args.type == "both":
			cprint(f"#fC#l{len(vods)} #fM#lVODSs#r & #fC#l{len(clips)} #fM#lClips#r")
			videos += vods
			videos += clips
		elif args.type == "vods":
			cprint(f"#fC#l{len(vods)} #fM#lVODSs#r")
			videos += vods
		elif args.type == "clips":
			cprint(f"#fC#l{len(clips)} #fM#lClips#r")
			videos += clips
			
	if args.type == "both":
		cprint(f"Total #fMVODs#r to download: #fC#l{totalvods}#r")
		cprint(f"Total #fMClips#r to download: #fC#l{totalclips}#r")
		cprint(f"Total #fM#lvideos#r to download: #fC#l{len(videos)}#r")
	elif args.type == "vods":
		cprint(f"Total #fMVODs#r to download: #fC#l{totalvods}#r")
	elif args.type == "clips":
		cprint(f"Total #fMClips#r to download: #fC#l{totalclips}#r")

	# Download all the videos we need.
	previouschannel = None
	for vod in videos:
		# Print if we're on to a new user.
		if previouschannel != vod.user_id:
			previouschannel = vod.user_id
			cprint(f"\nDownloading #fY#l{vod.user_name}#r's #fM#l{contentnoun}s#r...")

		# Generate path for video
		viddir = None
		contentnoun = None

		if isinstance(vod, Vod):
			viddir = voddir / vod.user_name.lower()
			contentnoun = "VOD"
		elif isinstance(vod, Clip):
			viddir = clipdir / vod.user_name.lower()
			contentnoun = "Clip"
		
		filename = viddir / f"{vod.created_at}_{vod.id}.mkv".replace(":", ";")
		filename = str(filename)
		metaname = str(viddir / (vod.id + ".meta"))

		# Write video data and handle exceptions
		try:
			if isinstance(vod, Vod): # Download video
				itd_dl.dl_video(vod, filename, metaname, 20)
			elif isinstance(vod, Clip): # Download clip
				itd_dl.dl_clip(vod, filename, metaname)
		except itd_dl.JoiningFailed:
			cprint(f"#fR#lVOD `{vod.id}` joining failed! Preserving files...#r")
		except itd_work.DownloadFailed:
			cprint(f"Download failed! Skipping...#r")
		except itd_work.DownloadCancelled:
			cprint(f"\n#fR#l{contentnoun} download cancelled.#r")
			raise KeyboardInterrupt()
	
	cprint("\n#fM#l* All done, goodbye! *#r\n")


def compare_existant_file(path, allvods):
	# Check for existing videos by finding the meta files
	existingvods = [f[:-5] for f in listdir(str(path)) if isfile(str(path/f)) and f[-4:]=="meta"]
	# Compare vods, if they arent downloaded (meta is missing) then we need to queue them
	result = [vod for vod in allvods if not any(vod.id == x for x in existingvods)]
	return result
