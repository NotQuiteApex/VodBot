use clap::ArgMatches;

pub fn run(args: &ArgMatches) {
	let pull_type = args.value_of("type").unwrap_or("both");
	println!("{}", pull_type);
	println!("pull command go!")
}