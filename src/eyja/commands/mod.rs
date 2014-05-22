use store::create_ram_store;

type MaybeArgs<'a> = Option<&'a[&'a str]>;

pub fn dispatch(command: &str, maybe_args: MaybeArgs) -> bool {
	let mut res = true;
	match command {
		"help" => help(),
		"test" => test(maybe_args),
		_ => res = false
	};
	res
}

pub fn help() {
	println!("Help TODO ;)");
}

pub fn test(maybe_args: MaybeArgs) {
	match maybe_args {
		None | Some([]) | Some([_]) => println!("Usage: test <key> <val>"),
		Some([key, val, ..]) => {
			let mut store = create_ram_store();
			store.set(key, val);
			let retval = store.get(key);
			println!("Val from store: {}", retval);
		}
	}
}