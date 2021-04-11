use clap::ArgMatches;

pub fn run(args: &ArgMatches) {
	if let Some(matches) = args.subcommand_matches("add") {

	} else if let Some(matches) = args.subcommand_matches("edit") {

	} else if let Some(matches) = args.subcommand_matches("list") {

	} else if let Some(matches) = args.subcommand_matches("rm") {

	}
	println!("stage command active!")
}