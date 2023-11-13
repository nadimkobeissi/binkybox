/* SPDX-FileCopyrightText: © 2023 Nadim Kobeissi <nadim@symbolic.software>
 * SPDX-License-Identifier: MIT */

use std::process::Command;
use std::thread;
use std::time;

pub fn open_url(url: &str) -> bool {
	if let Ok(mut child) = Command::new("cmd.exe")
		.arg("/C")
		.arg("start")
		.arg("")
		.arg(&url)
		.spawn()
	{
		thread::sleep(time::Duration::new(3, 0));
		if let Ok(status) = child.wait() {
			return status.success();
		}
	}
	false
}
