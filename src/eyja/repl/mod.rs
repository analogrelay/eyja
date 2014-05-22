use std;
use std::ascii::StrAsciiExt;
use std::vec::Vec;
use std::io::{IoError, EndOfFile};

pub fn run() {
	loop {
		print!("eyja> ");

		// Read the next line
		let line = match std::io::stdio::stdin().read_line() {
			Err(err) => match err.kind {
				EndOfFile => break,
				_ => io_err(err)
			},
			Ok(line) => line
		};

		let words = line.words().collect::<Vec<&str>>();

		match parse_line(words.as_slice()) {
			Some((command, maybe_args)) => {
				// Dispatch the command
				if command.eq_ignore_ascii_case("exit") {
					break
				} else {
					if !::commands::dispatch(command.to_ascii_lower(), maybe_args) {
						unknown_command(command);
					}
				}
			},
			_ => {}
		}
	}
	println!("Goodbye!");
}

fn unknown_command(command: &str) {
	println!("Unknown command: {}", command);
}

fn parse_line<'a>(words: &'a[&'a str]) -> Option<(&'a str, Option<&'a [&'a str]>)> {
	match words {
		[] => None,
		[command] => Some((command, None)),
		[command, ..args] => Some((command, Some(args)))
	}
}

fn io_err(err: IoError) -> ! {
	println!("I/O Error! {}", err);
	fail!();
}