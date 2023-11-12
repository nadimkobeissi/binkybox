/* SPDX-FileCopyrightText: © 2023 Nadim Kobeissi <nadim@symbolic.software>
 * SPDX-License-Identifier: MIT */

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::mpsc;

mod keys;
mod tray;

#[tokio::main]
async fn main() {
	let (tx, rx) = mpsc::sync_channel(1);
	let tx2 = tx.clone();
	tokio::spawn(keys::init(tx));
	tray::init(tx2, rx);
}
