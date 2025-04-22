use std::{fs::OpenOptions, os::windows::io::AsRawHandle};

use log::LevelFilter;
use simple_logger::SimpleLogger;
use windows::Win32::{
	Foundation::HANDLE,
	System::Console::{SetStdHandle, STD_ERROR_HANDLE},
};

pub fn init() {
	set_stderr();
	init_internal();

	log::info!("Stderr redirected into stderr.log");
}

fn set_stderr() {
	let file = OpenOptions::new()
		.create(true)
		.append(true)
		.open("stderr.log")
		.expect("Failed to open stderr.log");

	let handle = HANDLE(file.as_raw_handle());
	std::mem::forget(file);

	unsafe {
		// Redirect stderr
		if SetStdHandle(STD_ERROR_HANDLE, handle).is_err() {
			panic!("Failed to set stderr handle");
		}
	}
}

#[allow(dead_code)]
#[cfg(debug_assertions)]
fn init_internal() {
	SimpleLogger::new()
		.with_level(LevelFilter::Info)
		.env()
		.init()
		.unwrap();
}

#[allow(dead_code)]
#[cfg(not(debug_assertions))]
fn init_internal() {
	SimpleLogger::new()
		.with_level(LevelFilter::Error)
		.env()
		.init()
		.unwrap();
}
