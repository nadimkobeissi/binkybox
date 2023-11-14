/* SPDX-FileCopyrightText: © 2023 Nadim Kobeissi <nadim@symbolic.software>
 * SPDX-License-Identifier: MIT */

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod gui;
mod keys;
mod tray;
mod version;

#[tokio::main]
async fn main() {
	tokio::spawn(keys::init());
	tray::init();
}
