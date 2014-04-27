extern crate term;

mod repl;

fn main() {
	//println!("Creating memory-backed eyja store...");
	println!("Starting REPL loop ...");
	::repl::run();
}