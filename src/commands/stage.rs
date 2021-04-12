// Command to stage videos for slicing/upload

use clap::ArgMatches;
use serde_json::Value;

pub fn run(args: &ArgMatches, config: Value) {
	if let Some(matches) = args.subcommand_matches("add") {

	} else if let Some(matches) = args.subcommand_matches("edit") {

	} else if let Some(matches) = args.subcommand_matches("list") {

	} else if let Some(matches) = args.subcommand_matches("rm") {

	}
	println!("stage command active!")
}