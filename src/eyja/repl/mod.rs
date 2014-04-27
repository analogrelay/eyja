use std;
use std::ascii;
use std::char::is_whitespace;
use std::fmt::Show;
use std::result::Result;
use std::io::{IoError, EndOfFile};

enum Command {
	Exit,
	Invoke(|args: ~[&str]|)
}

enum ParseError {
	UnknownIoError(IoError),
	NoCommand,
	InvalidCommand(~str)
}

pub fn run() {
	loop {
		print!("eyja> ");

		// Parse a command
		match read_command() {
			Err(err) => on_error(err),
			Ok(command) => match command {
				Exit => break,
				Invoke(f) => f()
			}
		}
	}
	println!("")
	println!("Goodbye!")
}

fn read_command() -> Result<Command, ParseError> {
	match std::io::stdin().read_line() {
		Err(err) => {
			match err.kind {
				EndOfFile => Ok(Exit),
				_ => Err(UnknownIoError(err))
			}
		},
		Ok(line) => parse_command(line)
	}
}

fn help() {
	println!("TODO: Help ;P")
}

fn parse_command(line : &str) -> Result<Command, ParseError> {
	// Split!
	match line.words().collect() {
		[] => Err(NoCommand),
		[command] => Ok(build_command(command)),
		[command, ..args] => Ok(build_command(command))
	}
}

fn build_command(command : &str) -> Command {
	// Normalize the command
	if !command.is_ascii() {
		Err(InvalidCommand(command))
	} else {
		match command.to_ascii().to_lower() {
			"exit" => Ok(Exit),
			"help" => Ok(Invoke(help))
		}
	}
}

fn on_error(err: ParseError) {
	println!("** Error: {} **", err)
}