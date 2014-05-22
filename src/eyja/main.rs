#![feature(default_type_params)]

extern crate term;
extern crate collections;

mod store;
mod repl;

fn main() {
	//println!("Creating memory-backed eyja store...");
	println!("Starting REPL loop ...");
	repl::run();
}