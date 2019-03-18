use std::sync::Arc;

use crate::servers::Server;
use std::thread;
use std::time;

pub struct Lcd {}

impl Lcd {
	pub fn new() -> Lcd {
		Lcd {}
	}

	pub fn run(&mut self, server: Arc<Server>) {
		let stats = server.get_server_stats().unwrap();

		loop {
			thread::sleep(time::Duration::from_millis(1000));
			stats.head.height.to_string();
			//			let mut file = LineWriter::new(file);
			//			file.write_all(concat!(stats.head.height.to_string(), "\n"));
		}
	}
}
