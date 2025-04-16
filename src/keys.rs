/* SPDX-FileCopyrightText: © 2023 Nadim Kobeissi <nadim@symbolic.software>
 * SPDX-License-Identifier: MIT */

use inputbot::KeybdKey::{self, *};
use json::JsonValue;
use regex::Regex;
use std::collections::HashMap;
use lazy_static::lazy_static;

use windows::{
    Win32::{
        Foundation::*,
        UI::WindowsAndMessaging::*,
        UI::Controls::STATE_SYSTEM_INVISIBLE,
    },
};

use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

use crate::config;

lazy_static! {
	static ref KEY_MAP: HashMap<&'static str, KeybdKey> = [
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
	.collect();
}

pub async fn init() {
	bind_shortcuts();
	inputbot::handle_input_events();
}

pub fn bind_shortcuts() {
	let my_config = config::read();
	for (_, value) in KEY_MAP.iter() {
		value.unbind();
	}
	for i in 0..8 {
		let shortcut = process_shortcut(&my_config, i);
		if let Some(key_to_bind) = shortcut.get(shortcut.len().saturating_sub(1)) {
			key_to_bind.blockable_bind(move || {
				if shortcut
					.iter()
					.take(shortcut.len() - 1)
					.all(|key| key.is_pressed())
				{
					for (_, value) in KEY_MAP.iter() {
						if value.is_pressed() && !shortcut.contains(value) {
							return inputbot::BlockInput::DontBlock;
						}
					}
					switch_to_desktop(i, 0);
					return inputbot::BlockInput::Block;
				}
				return inputbot::BlockInput::DontBlock;
			});
		}
	}
}

unsafe extern "system" fn enum_windows_and_switch_app_focus(hwnd: HWND, _lparam: LPARAM) -> BOOL {
    // remove not visible windows
    if IsWindowVisible(hwnd) == false {
        return BOOL(1);
    }

    // remove windows with invisible title bar
    let mut ti = TITLEBARINFO {
        cbSize: std::mem::size_of::<TITLEBARINFO>() as u32,
        rcTitleBar: RECT {left: 0, top: 0, right: 0, bottom: 0},
        rgstate: [0; 6],
    };
    let _ = GetTitleBarInfo(hwnd, &mut ti);
    if ti.rgstate[0] & STATE_SYSTEM_INVISIBLE.0 > 0 {
        return BOOL(1);
    }

    // remove "floating toolbar" windows that are not visible in alt+tab
    if WINDOW_EX_STYLE(GetWindowLongW(hwnd, GWL_EXSTYLE).try_into().unwrap()) & WS_EX_TOOLWINDOW != WINDOW_EX_STYLE(0) {
        return BOOL(1);
    }

    // remove windows with empty title bar
    // + Settings window, for some reason this window is focused on empty desktops even if it is
    //   not opened or visible in alt+tab or taskbar
    let mut buffer: [u16; 256] = [0; 256];
    GetWindowTextW(hwnd, &mut buffer);
    let window_title = OsString::from_wide(&buffer).to_string_lossy().into_owned();
    if window_title.is_empty() || window_title.contains("Settings") {
        return BOOL(1)
    }

    // remove windows that are not in the current virtual desktop
    let is_on_current_desktop = winvd::is_window_on_current_desktop(hwnd as windows::Win32::Foundation::HWND).unwrap();
    if !is_on_current_desktop {
        return BOOL(1)
    }

    let _ = SetForegroundWindow(hwnd);
    return BOOL(0); // Stop enumeration
}

fn switch_to_desktop(desktop: u32, tries: u8) {
	if tries <= 10 {
		match winvd::switch_desktop(desktop) {
				Ok(_) => {
                    unsafe {
                        let _ = EnumWindows(Some(enum_windows_and_switch_app_focus), LPARAM {0: 0} );
                    }
                }
				Err(_) => {
					match winvd::create_desktop() {
						Ok(_) => {
							switch_to_desktop(desktop, tries + 1);
						}
						Err(e) => {
							println!("Error: {:?}", e);
					}
				}
			}
		}
	}
}

fn process_shortcut(config: &JsonValue, desktop: u32) -> Vec<KeybdKey> {
	let desktop_str = format!("desktop_{}", desktop + 1);
	let shortcut_sanitized =
		sanitize_keyboard_shortcut(config["shortcuts"][&desktop_str].to_string());
	let shortcut_string = match check_keyboard_shortcut(shortcut_sanitized.clone()) {
		true => shortcut_sanitized,
		false => config::get_default()["shortcuts"][&desktop_str].to_string(),
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

fn build_keyboard_shortcut(input: &str) -> Vec<KeybdKey> {
	input
		.split('+')
		.filter_map(|part| KEY_MAP.get(&part))
		.cloned()
		.collect()
}
