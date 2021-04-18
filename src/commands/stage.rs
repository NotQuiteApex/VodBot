// Command to stage videos for slicing/upload

use clap::ArgMatches;
use serde_json::Value;

pub fn run(args: &ArgMatches, _config: Value) {
	if let Some(_matches) = args.subcommand_matches("add") {

	} else if let Some(_matches) = args.subcommand_matches("edit") {

	} else if let Some(_matches) = args.subcommand_matches("list") {

	} else if let Some(_matches) = args.subcommand_matches("rm") {

	}
	println!("stage command active!")
}