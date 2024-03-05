/* SPDX-FileCopyrightText: © 2023 Nadim Kobeissi <nadim@symbolic.software>
 * SPDX-License-Identifier: MIT */

use inputbot::KeybdKey::{self, *};
use json::JsonValue;
use regex::Regex;
use std::collections::HashMap;
use winvd::{switch_desktop, get_desktop_count};

use crate::config;

pub async fn init() {
	bind_shortcuts();
	inputbot::handle_input_events();
}

pub fn bind_shortcuts() {
	let my_config = config::read();
	for (_, value) in key_map() {
		value.unbind();
	}
	for i in 0..10 {
		let desktop = format!("desktop_{}", i + 1);
		let shortcut = process_shortcut(&my_config, &desktop);
		if let Some(key_to_bind) = shortcut.get(shortcut.len().saturating_sub(1)) {
			key_to_bind.blockable_bind(move || {
				if shortcut
					.iter()
					.take(shortcut.len() - 1)
					.all(|key| key.is_pressed())
				{
					// Ignore desktops that aren't configured.
					if i < get_desktop_count().unwrap() {
						switch_desktop(i as i32).unwrap();
					}
					return inputbot::BlockInput::Block;
				}
				return inputbot::BlockInput::DontBlock;
			});
		}
	}
}

fn process_shortcut(config: &JsonValue, desktop: &str) -> Vec<KeybdKey> {
	let shortcut_sanitized =
		sanitize_keyboard_shortcut(config["shortcuts"][desktop].to_string());
	let shortcut_string = match check_keyboard_shortcut(shortcut_sanitized.clone()) {
		true => shortcut_sanitized,
		false => config::get_default()["shortcuts"][desktop].to_string(),
	};
	build_keyboard_shortcut(shortcut_string.as_str())
}

pub fn sanitize_keyboard_shortcut(input: String) -> String {
	let mut input = input.to_uppercase();
	input.retain(|c| !c.is_whitespace());
	return input;
}

pub fn check_keyboard_shortcut(input: String) -> bool {
	let re = Regex::new(
		r"^((([LR]CTRL)|([LR]ALT)|([LR]WIN)|([LR]SHIFT)|(F\d))\+){1,4}([A-Z\d]|(F\d))$",
	);
	if re.is_err() {
		return false;
	}
	return re.unwrap().is_match(&input);
}

fn key_map() -> HashMap<&'static str, KeybdKey> {
	[
		("CTRL", LControlKey),
		("LCTRL", LControlKey),
		("RCTRL", RControlKey),
		("ALT", LAltKey),
		("LALT", LAltKey),
		("RALT", RAltKey),
		("WIN", LSuper),
		("LWIN", LSuper),
		("RWIN", RSuper),
		("SHIFT", LShiftKey),
		("LSHIFT", LShiftKey),
		("RSHIFT", RShiftKey),
		("F1", F1Key),
		("F2", F2Key),
		("F3", F3Key),
		("F4", F4Key),
		("F5", F5Key),
		("F6", F6Key),
		("F7", F7Key),
		("F8", F8Key),
		("F9", F9Key),
		("F10", F10Key),
		("F11", F11Key),
		("F12", F12Key),
		("A", AKey),
		("B", BKey),
		("C", CKey),
		("D", DKey),
		("E", EKey),
		("F", FKey),
		("G", GKey),
		("H", HKey),
		("I", IKey),
		("J", JKey),
		("K", KKey),
		("L", LKey),
		("M", MKey),
		("N", NKey),
		("O", OKey),
		("P", PKey),
		("Q", QKey),
		("R", RKey),
		("S", SKey),
		("T", TKey),
		("U", UKey),
		("V", VKey),
		("W", WKey),
		("X", XKey),
		("Y", YKey),
		("Z", ZKey),
		("1", Numrow1Key),
		("2", Numrow2Key),
		("3", Numrow3Key),
		("4", Numrow4Key),
		("5", Numrow5Key),
		("6", Numrow6Key),
		("7", Numrow7Key),
		("8", Numrow8Key),
		("9", Numrow9Key),
		("0", Numrow0Key),
	]
	.iter()
	.cloned()
	.collect()
}

fn build_keyboard_shortcut(input: &str) -> Vec<KeybdKey> {
	let key_map = key_map();
	input
		.split('+')
		.filter_map(|part| key_map.get(&part))
		.cloned()
		.collect()
}
