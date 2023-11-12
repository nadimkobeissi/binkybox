/* SPDX-FileCopyrightText: © 2023 Nadim Kobeissi <nadim@symbolic.software>
 * SPDX-License-Identifier: MIT */

use std::sync::mpsc;
use tray_item::{IconSource, TrayItem};

mod gui;
pub enum TrayMessage {
	Settings,
	Quit,
	SetIcon1,
	SetIcon2,
	SetIcon3,
	SetIcon4
}

pub fn init(tx: mpsc::SyncSender<TrayMessage>, rx: mpsc::Receiver<TrayMessage>) {
	let mut tray = TrayItem::new(
		"BinkyBox",
		IconSource::Resource("icon"),
	)
	.unwrap();

	let tx2 = tx.clone();

	tray.add_menu_item("Settings", move || {
		tx.send(TrayMessage::Settings).unwrap();
	})
	.unwrap();

	tray.inner_mut().add_separator().unwrap();

	tray.add_menu_item("Quit", move || {
		tx2.send(TrayMessage::Quit).unwrap();
	})
	.unwrap();

	loop {
		match rx.recv() {
			Ok(TrayMessage::Settings) => {
				gui::init();
			}
			Ok(TrayMessage::Quit) => {
				std::process::exit(0);
			}
			Ok(TrayMessage::SetIcon1) => {
				tray.set_icon(IconSource::Resource("num_1")).unwrap();
			}
			Ok(TrayMessage::SetIcon2) => {
				tray.set_icon(IconSource::Resource("num_2")).unwrap();
			}
			Ok(TrayMessage::SetIcon3) => {
				tray.set_icon(IconSource::Resource("num_3")).unwrap();
			}
			Ok(TrayMessage::SetIcon4) => {
				tray.set_icon(IconSource::Resource("num_4")).unwrap();
			}
			_ => {}
		}
	}
}

pub fn set_icon_num(tx: mpsc::SyncSender<TrayMessage>, num: u8) {
	match num {
		0 => tx.send(TrayMessage::SetIcon1).unwrap(),
		1 => tx.send(TrayMessage::SetIcon2).unwrap(),
		2 => tx.send(TrayMessage::SetIcon3).unwrap(),
		3 => tx.send(TrayMessage::SetIcon4).unwrap(),
		_ => {}
	}
}