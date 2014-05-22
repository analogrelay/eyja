use std::fmt;
use std::fmt::{Show, Formatter};
use std::hash::sip::SipHasher;
use collections::hashmap::HashMap;

pub trait DataStore {
	fn set(&mut self, key: ~str, val: ~str);
	fn get<'a>(&'a self, key: &str) -> Option<&'a str>;
}

pub struct RamStore {
	map: HashMap<~str, ~str, SipHasher>
}

impl DataStore for RamStore {
	fn set(&mut self, key: ~str, val: ~str) {
		self.map.insert(key, val);
	}

	fn get<'a>(&'a self, key: &str) -> Option<&'a str> {
		// Get a reference the value that matches the input
		match self.map.find_equiv(&key) {
			Some(s) => Some(s.as_slice()),
			None => None
		}
	}
}

impl RamStore {
	pub fn create() -> RamStore {
		RamStore { map: HashMap::new() }
	}
}

pub struct Database {
	store: Box<S>
}

impl Database {
	pub fn create(store: Box<DataStore>) -> Database {
		Database { store: store }
	}

	pub fn set(&mut self, key: &str, val: &str) {
		// Copy the values in to new owned boxes and store them in the data store
		self.store.set(key.to_owned(), val.to_owned());
	}

	pub fn get<'a>(&'a self, key: &str) -> Option<&'a str> {
		// Get the key that matches the box
		self.store.get(key)
	}
}

pub fn create_ram_store() -> Database {
	Database::create(box RamStore::create())
}
