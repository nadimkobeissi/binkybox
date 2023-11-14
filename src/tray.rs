/* SPDX-FileCopyrightText: © 2023 Nadim Kobeissi <nadim@symbolic.software>
 * SPDX-License-Identifier: MIT */

use std::sync::mpsc;
use tray_item::{IconSource, TrayItem};
use winvd::DesktopEvent;

use crate::gui;
pub enum TrayMessage {
	Settings,
	Quit,
}

pub fn init() {
	let mut tray = TrayItem::new("BinkyBox", IconSource::Resource("icon")).unwrap();
	let (tx, rx) = mpsc::sync_channel(1);
	let tx_0 = tx.to_owned();
	tray.add_menu_item("Settings", move || {
		tx.send(TrayMessage::Settings).unwrap();
	})
	.unwrap();
	tray.inner_mut().add_separator().unwrap();
	tray.add_menu_item("Quit", move || {
		tx_0.send(TrayMessage::Quit).unwrap();
	})
	.unwrap();
	tokio::spawn(icon_change_listener(tray));
	loop {
		match rx.recv() {
			Ok(TrayMessage::Settings) => {
				gui::init();
			}
			Ok(TrayMessage::Quit) => {
				std::process::exit(0);
			}
			_ => {}
		}
	}
}

async fn icon_change_listener(mut tray: TrayItem) {
	let (tx, rx) = std::sync::mpsc::channel::<DesktopEvent>();
	let _notifications_thread = winvd::listen_desktop_events(tx);
	for event in rx {
		match event {
			DesktopEvent::DesktopChanged { new: n, old: _ } => match n.get_index() {
				Ok(0) => tray.set_icon(IconSource::Resource("num_1")).unwrap(),
				Ok(1) => tray.set_icon(IconSource::Resource("num_2")).unwrap(),
				Ok(2) => tray.set_icon(IconSource::Resource("num_3")).unwrap(),
				Ok(3) => tray.set_icon(IconSource::Resource("num_4")).unwrap(),
				_ => {}
			},
			_ => {}
		}
	}
}
