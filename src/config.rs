/* SPDX-FileCopyrightText: © 2023 Nadim Kobeissi <nadim@symbolic.software>
 * SPDX-License-Identifier: MIT */

use json::object;
use json::JsonValue;
use std::fs::File;
use std::io::Error;
use std::io::Read;
use std::io::Write;

pub fn get_default() -> JsonValue {
	object! {
		shortcuts: {
			desktop_1: "LALT+LSHIFT+1",
			desktop_2: "LALT+LSHIFT+2",
			desktop_3: "LALT+LSHIFT+3",
			desktop_4: "LALT+LSHIFT+4",
			desktop_5: "LALT+LSHIFT+5",
			desktop_6: "LALT+LSHIFT+6",
			desktop_7: "LALT+LSHIFT+7",
			desktop_8: "LALT+LSHIFT+8",
		}
	}
}

pub fn write(input: JsonValue) -> Result<bool, Error> {
	let my_config = input;
	let config_string = json::stringify(my_config);
	let mut file = File::create("binkybox.config.json")?;
	file.write_all(config_string.as_bytes())?;
	return Result::Ok(true);
}

pub fn read() -> JsonValue {
	match File::open("binkybox.config.json") {
		Ok(mut file) => {
			let mut config_string = String::new();
			match file.read_to_string(&mut config_string) {
				Ok(_) => match json::parse(&config_string) {
					Ok(my_config) => my_config,
					Err(_) => get_default(),
				},
				Err(_) => get_default(),
			}
		}
		Err(_) => get_default(),
	}
}
