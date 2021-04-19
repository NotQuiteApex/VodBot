// Command to stage videos for slicing/upload

use super::super::util;


pub fn run(args: &clap::ArgMatches, _config: serde_json::Value) -> Result<(), util::ExitMsg> {
	if let Some(_matches) = args.subcommand_matches("add") {

	} else if let Some(_matches) = args.subcommand_matches("edit") {

	} else if let Some(_matches) = args.subcommand_matches("list") {

	} else if let Some(_matches) = args.subcommand_matches("rm") {

	}
	println!("stage command active!");

	Ok(())
}