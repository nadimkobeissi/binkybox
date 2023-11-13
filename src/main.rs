/* SPDX-FileCopyrightText: © 2023 Nadim Kobeissi <nadim@symbolic.software>
 * SPDX-License-Identifier: MIT */

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::mpsc;

mod config;
mod gui;
mod keys;
mod tray;
mod version;

#[tokio::main]
async fn main() {
	let (tx_tray, rx_tray) = mpsc::sync_channel(1);
	let tx_keys = tx_tray.clone();
	tokio::spawn(keys::init(tx_keys));
	tray::init(tx_tray, rx_tray);
}
