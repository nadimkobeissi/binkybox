/* SPDX-FileCopyrightText: © 2023 Nadim Kobeissi <nadim@symbolic.software>
 * SPDX-License-Identifier: MIT */


use inputbot::KeybdKey::*;
use winvd::switch_desktop;
use std::sync::mpsc;
use regex::Regex;

use crate::tray;

pub async fn init(tx: mpsc::SyncSender<tray::TrayMessage>) {
	let tx0 = tx.clone();
	let tx1 = tx.clone();
	let tx2 = tx.clone();
	let tx3 = tx.clone();
	Numrow1Key.bind(move || {
		if LControlKey.is_pressed() && LAltKey.is_pressed() {
			let tx0c = tx0.clone();
			tray::set_icon_num(tx0c, 0);
			switch_desktop(0).unwrap();
		}
	});
	Numrow2Key.bind(move || {
		if LControlKey.is_pressed() && LAltKey.is_pressed() {
			let tx1c = tx1.clone();
			tray::set_icon_num(tx1c, 1);
			switch_desktop(1).unwrap();
		}
	});
	Numrow3Key.bind(move || {
		if LControlKey.is_pressed() && LAltKey.is_pressed() {
			let tx2c = tx2.clone();
			tray::set_icon_num(tx2c, 2);
			switch_desktop(2).unwrap();
		}
	});
	Numrow4Key.bind(move || {
		if LControlKey.is_pressed() && LAltKey.is_pressed() {
			let tx3c = tx3.clone();
			tray::set_icon_num(tx3c, 3);
			switch_desktop(3).unwrap();
		}
	});
	inputbot::handle_input_events();
}

fn check_keyboard_shortcut(input: String) -> bool {
	let re = Regex::new(r"^((([LR]Ctrl)|([LR]Alt)|([LR]Win)|([LR]Shift)|[A-Z\d])\+){0,4}(([LR]Ctrl)|([LR]Alt)|([LR]Win)|([LR]Shift)|[A-Z\d])$");
	if re.is_err() {
		return false;
	}
	return re.unwrap().is_match(&input);
}

fn deconstruct_keyboard_shortcut(input: String) {
	let parts = input.split("+");
	for part in parts {
		match part {
			"LCtrl" => {
				println!("LCtrl");
			}
			"RCtrl" => {
				println!("RCtrl");
			}
			"LAlt" => {
				println!("LAlt");
			}
			"RAlt" => {
				println!("RAlt");
			}
			"LWin" => {
				println!("LWin");
			}
			"RWin" => {
				println!("RWin");
			}
			"LShift" => {
				println!("LShift");
			}
			"RShift" => {
				println!("RShift");
			}
			_ => {
				println!("{}", part);
			}
		}
	}
}