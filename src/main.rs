pub mod config;
mod main_line;
mod spy_line;
pub mod utils;

use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
	let main_line = Arc::new(Mutex::new(main_line::MainLine::default()));
	let spy_line = Arc::new(Mutex::new(spy_line::SpyLine::default()));

	let _ = tokio::spawn(main_line::MainLine::websocket(main_line.clone()));
	let mut cycle = 0;
	loop {
		// start collecting all lines simultaneously
		let main_line_handler = main_line::MainLine::collect(main_line.clone());
		// ...

		// Await everything
		let _ = main_line_handler.await;
		// ...

		// Display everything
		println!("{}", main_line.lock().unwrap().display());

		cycle += 1;
		if cycle == 16 {
			cycle = 1; // rolls to 1, so I can make special cases for 0
		}
		tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
	}
}
