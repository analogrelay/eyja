use std;
use std::ascii::StrAsciiExt;
use std::vec::Vec;
use std::io::{IoError, EndOfFile};
use std::hash::sip::SipHasher;
use collections::hashmap::HashMap;

use store::create_ram_store;
use self::context::ReplContext;

mod context;

type MaybeArgs<'a> = Option<&'a[&'a str]>;

pub fn run() {
	let ctx = ReplContext::new();
	let cmds = HashMap::<~str, fn (args: MaybeArgs, ctx: &ReplContext)>::new();
	cmds.insert("help", help);

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
					match cmds.find_equiv(command) {
						Some(f) => f(maybe_args, &ctx),
						None => unknown_command(command)
					}
				}
			},
			_ => {}
		}
	}
	println!("Goodbye!");
}

fn help(_ : MaybeArgs, ctx: &mut ReplContext) {
	println!("Active Database: {}", ctx.db());
	println!("Help TODO ;)");
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