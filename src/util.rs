// Utility functions

// Exit codes for different situations.
pub enum ExitStat {
	_CleanExit,
	MissingHomeDirectory,
}

pub fn exit_prog(i: ExitStat, msg: Option<&str>) {
	let exit_code = i as i32;
	match msg {
		Some(message) => {
			println!("Error: ({}) {}. Exiting...", exit_code, message);
			std::process::exit(exit_code);
		},
		None => {
			println!("Exiting...");
			std::process::exit(exit_code);
		}
	}
}