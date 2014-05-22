use store::Database;

struct ReplContext {
	_db: Option<Box<Database>>
}

impl ReplContext {
	fn new() -> ReplContext {
		
	}

	fn open(&mut self, db: Box<Database>) {
		self.store = db;
	}

	fn db(&self) -> Option<&Database> {
		match self._db {
			Some(ref db) => Some(db),
			None => None
		}
	}
}