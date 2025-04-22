/* SPDX-FileCopyrightText: © 2023 Nadim Kobeissi <nadim@symbolic.software>
 * SPDX-License-Identifier: MIT */

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod gui;
mod keys;
mod logging;
mod tray;
mod version;

use windows::Win32::System::Console::{AllocConsole, FreeConsole};

#[tokio::main]
async fn main() {
	// Establish this app as foreground capable application so it can use SetForegroundWindow
	// Create gui console and immediately close it
	unsafe {
		let _ = AllocConsole();
		let _ = FreeConsole();
	}

	logging::init();
	tokio::spawn(keys::init());
	tray::init();
}
