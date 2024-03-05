/* SPDX-FileCopyrightText: © 2023 Nadim Kobeissi <nadim@symbolic.software>
 * SPDX-License-Identifier: MIT */

use json::object;
use open;
use winsafe::{self as w, co, gui, prelude::*};

use crate::config;
use crate::keys;
use crate::version;

#[allow(dead_code)]
#[derive(Clone)]
pub struct MainWindow {
	wnd: gui::WindowMain,
	tab_ctrl: gui::Tab,
}

impl MainWindow {
	pub fn new() -> Self {
		let wnd = gui::WindowMain::new(gui::WindowMainOpts {
			title: "BinkyBox Settings".to_owned(),
			class_icon: gui::Icon::Str(winsafe::WString::from_str("icon")),
			size: (300, 690),
			style: gui::WindowMainOpts::default().style | co::WS::MINIMIZEBOX,
			..Default::default()
		});
		let tab_ctrl = gui::Tab::new(
			&wnd,
			gui::TabOpts {
				position: (10, 10),
				size: (280, 690),
				items: vec![
					(
						"Keyboard Shortcuts".to_owned(),
						Box::new(TabContainerKeyboardShortcuts::new(&wnd)),
					),
					("About".to_owned(), Box::new(TabContainerAbout::new(&wnd))),
				],
				..Default::default()
			},
		);
		let new_self = Self { wnd, tab_ctrl };
		new_self.events();
		new_self
	}

	pub fn run(&self) -> w::AnyResult<i32> {
		self.wnd.run_main(None)
	}

	fn events(&self) {}
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct TabContainerKeyboardShortcuts {
	wnd: gui::WindowControl,
	lbl_0: gui::Label,
	txt_0: gui::Edit,
	lbl_1: gui::Label,
	txt_1: gui::Edit,
	lbl_2: gui::Label,
	txt_2: gui::Edit,
	lbl_3: gui::Label,
	txt_3: gui::Edit,
	lbl_4: gui::Label,
	txt_4: gui::Edit,
	lbl_5: gui::Label,
	txt_5: gui::Edit,
	lbl_6: gui::Label,
	txt_6: gui::Edit,
	lbl_7: gui::Label,
	txt_7: gui::Edit,
	lbl_8: gui::Label,
	txt_8: gui::Edit,
	lbl_9: gui::Label,
	txt_9: gui::Edit,
	btn_set: gui::Button,
	btn_help: gui::Button,
}

impl GuiTab for TabContainerKeyboardShortcuts {
	fn as_ctrl(&self) -> &gui::WindowControl {
		&self.wnd
	}
}

impl TabContainerKeyboardShortcuts {
	pub fn new(parent: &impl GuiParent) -> Self {
		let my_config = config::read();
		let wnd = gui::WindowControl::new(
			parent,
			gui::WindowControlOpts {
				ex_style: co::WS_EX::CONTROLPARENT,
				..Default::default()
			},
		);
		let lbl_0 = gui::Label::new(
			&wnd,
			gui::LabelOpts {
				position: (20, 20),
				text: "Switch to Desktop 1".to_owned(),
				..Default::default()
			},
		);
		let txt_0 = gui::Edit::new(
			&wnd,
			gui::EditOpts {
				position: (20, 40),
				width: 180,
				text: my_config["shortcuts"]["desktop_1"].to_string().to_owned(),
				..Default::default()
			},
		);
		let lbl_1 = gui::Label::new(
			&wnd,
			gui::LabelOpts {
				position: (20, 80),
				text: "Switch to Desktop 2".to_owned(),
				..Default::default()
			},
		);
		let txt_1 = gui::Edit::new(
			&wnd,
			gui::EditOpts {
				position: (20, 100),
				width: 180,
				text: my_config["shortcuts"]["desktop_2"].to_string().to_owned(),
				..Default::default()
			},
		);
		let lbl_2 = gui::Label::new(
			&wnd,
			gui::LabelOpts {
				position: (20, 140),
				text: "Switch to Desktop 3".to_owned(),
				..Default::default()
			},
		);
		let txt_2 = gui::Edit::new(
			&wnd,
			gui::EditOpts {
				position: (20, 160),
				width: 180,
				text: my_config["shortcuts"]["desktop_3"].to_string().to_owned(),
				..Default::default()
			},
		);
		let lbl_3 = gui::Label::new(
			&wnd,
			gui::LabelOpts {
				position: (20, 200),
				text: "Switch to Desktop 4".to_owned(),
				..Default::default()
			},
		);
		let txt_3 = gui::Edit::new(
			&wnd,
			gui::EditOpts {
				position: (20, 220),
				width: 180,
				text: my_config["shortcuts"]["desktop_4"].to_string().to_owned(),
				..Default::default()
			},
		);
		let lbl_4 = gui::Label::new(
			&wnd,
			gui::LabelOpts {
				position: (20, 260),
				text: "Switch to Desktop 5".to_owned(),
				..Default::default()
			},
		);
		let txt_4 = gui::Edit::new(
			&wnd,
			gui::EditOpts {
				position: (20, 280),
				width: 180,
				text: my_config["shortcuts"]["desktop_5"].to_string().to_owned(),
				..Default::default()
			},
		);
		let lbl_5 = gui::Label::new(
			&wnd,
			gui::LabelOpts {
				position: (20, 320),
				text: "Switch to Desktop 6".to_owned(),
				..Default::default()
			},
		);
		let txt_5 = gui::Edit::new(
			&wnd,
			gui::EditOpts {
				position: (20, 340),
				width: 180,
				text: my_config["shortcuts"]["desktop_6"].to_string().to_owned(),
				..Default::default()
			},
		);
		let lbl_6 = gui::Label::new(
			&wnd,
			gui::LabelOpts {
				position: (20, 380),
				text: "Switch to Desktop 7".to_owned(),
				..Default::default()
			},
		);
		let txt_6 = gui::Edit::new(
			&wnd,
			gui::EditOpts {
				position: (20, 400),
				width: 180,
				text: my_config["shortcuts"]["desktop_7"].to_string().to_owned(),
				..Default::default()
			},
		);
		let lbl_7 = gui::Label::new(
			&wnd,
			gui::LabelOpts {
				position: (20, 440),
				text: "Switch to Desktop 8".to_owned(),
				..Default::default()
			},
		);
		let txt_7 = gui::Edit::new(
			&wnd,
			gui::EditOpts {
				position: (20, 460),
				width: 180,
				text: my_config["shortcuts"]["desktop_8"].to_string().to_owned(),
				..Default::default()
			},
		);
		let lbl_8 = gui::Label::new(
			&wnd,
			gui::LabelOpts {
				position: (20, 500),
				text: "Switch to Desktop 9".to_owned(),
				..Default::default()
			},
		);
		let txt_8 = gui::Edit::new(
			&wnd,
			gui::EditOpts {
				position: (20, 520),
				width: 180,
				text: my_config["shortcuts"]["desktop_9"].to_string().to_owned(),
				..Default::default()
			},
		);
		let lbl_9 = gui::Label::new(
			&wnd,
			gui::LabelOpts {
				position: (20, 560),
				text: "Switch to Desktop 10".to_owned(),
				..Default::default()
			},
		);
		let txt_9 = gui::Edit::new(
			&wnd,
			gui::EditOpts {
				position: (20, 580),
				width: 180,
				text: my_config["shortcuts"]["desktop_10"].to_string().to_owned(),
				..Default::default()
			},
		);
		let btn_set = gui::Button::new(
			&wnd,
			gui::ButtonOpts {
				position: (20, 620),
				width: 100,
				text: "Set".to_owned(),
				..Default::default()
			},
		);
		let btn_help = gui::Button::new(
			&wnd,
			gui::ButtonOpts {
				position: (150, 620),
				width: 50,
				text: "Help".to_owned(),
				..Default::default()
			},
		);
		let new_self = Self {
			wnd,
			lbl_0,
			txt_0,
			lbl_1,
			txt_1,
			lbl_2,
			txt_2,
			lbl_3,
			txt_3,
			lbl_4,
			txt_4,
			lbl_5,
			txt_5,
			lbl_6,
			txt_6,
			lbl_7,
			txt_7,
			lbl_8,
			txt_8,
			lbl_9,
			txt_9,
			btn_set,
			btn_help,
		};
		new_self.events();
		new_self
	}
	fn events(&self) {
		let self_0 = self.clone();
		self.btn_set.on().bn_clicked(move || {
			for i in 0..10 {
				let shortcut_sanitized = keys::sanitize_keyboard_shortcut(match i {
					0 => self_0.txt_0.text(),
					1 => self_0.txt_1.text(),
					2 => self_0.txt_2.text(),
					3 => self_0.txt_3.text(),
					4 => self_0.txt_4.text(),
					5 => self_0.txt_5.text(),
					6 => self_0.txt_6.text(),
					7 => self_0.txt_7.text(),
					8 => self_0.txt_8.text(),
					_ => self_0.txt_9.text(),
				});
				if !keys::check_keyboard_shortcut(shortcut_sanitized) {
					w::task_dlg::error(
						&self_0.wnd.hwnd().GetParent()?,
						"Error",
						None,
						&format!(
							"Keyboard shortcut for \"Switch to Desktop {}\" is invalid.",
							i + 1
						),
					)?;
					return Ok(());
				}
			}
			self_0
				.txt_0
				.set_text(keys::sanitize_keyboard_shortcut(self_0.txt_0.text()).as_str());
			self_0
				.txt_1
				.set_text(keys::sanitize_keyboard_shortcut(self_0.txt_1.text()).as_str());
			self_0
				.txt_2
				.set_text(keys::sanitize_keyboard_shortcut(self_0.txt_2.text()).as_str());
			self_0
				.txt_3
				.set_text(keys::sanitize_keyboard_shortcut(self_0.txt_3.text()).as_str());
			self_0
				.txt_4
				.set_text(keys::sanitize_keyboard_shortcut(self_0.txt_4.text()).as_str());
			self_0
				.txt_5
				.set_text(keys::sanitize_keyboard_shortcut(self_0.txt_5.text()).as_str());
			self_0
				.txt_6
				.set_text(keys::sanitize_keyboard_shortcut(self_0.txt_6.text()).as_str());
			self_0
				.txt_7
				.set_text(keys::sanitize_keyboard_shortcut(self_0.txt_7.text()).as_str());
			self_0
				.txt_8
				.set_text(keys::sanitize_keyboard_shortcut(self_0.txt_8.text()).as_str());
			self_0
				.txt_9
				.set_text(keys::sanitize_keyboard_shortcut(self_0.txt_9.text()).as_str());
			config::write(object! {
				shortcuts: {
					desktop_1:  self_0.txt_0.text(),
					desktop_2:  self_0.txt_1.text(),
					desktop_3:  self_0.txt_2.text(),
					desktop_4:  self_0.txt_3.text(),
					desktop_5:  self_0.txt_4.text(),
					desktop_6:  self_0.txt_5.text(),
					desktop_7:  self_0.txt_6.text(),
					desktop_8:  self_0.txt_7.text(),
					desktop_9:  self_0.txt_8.text(),
					desktop_10: self_0.txt_9.text(),
				}
			})
			.unwrap();
			keys::bind_shortcuts();
			w::task_dlg::info(
				&self_0.wnd.hwnd().GetParent()?,
				"Success",
				None,
				"Keyboard shortcuts saved.",
			)?;
			Ok(())
		});
		self.btn_help.on().bn_clicked(move || {
			open::that(
				"https://github.com/nadimkobeissi/binkybox#setting-keyboard-shortcuts",
			)
			.unwrap();
			Ok(())
		});
	}
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct TabContainerAbout {
	wnd: gui::WindowControl,
	lbl_0: gui::Label,
	lbl_1: gui::Label,
	btn: gui::Button,
}

impl GuiTab for TabContainerAbout {
	fn as_ctrl(&self) -> &gui::WindowControl {
		&self.wnd
	}
}

impl TabContainerAbout {
	pub fn new(parent: &impl GuiParent) -> Self {
		let wnd = gui::WindowControl::new(
			parent,
			gui::WindowControlOpts {
				ex_style: co::WS_EX::CONTROLPARENT,
				..Default::default()
			},
		);
		let lbl_0 = gui::Label::new(
			&wnd,
			gui::LabelOpts {
				position: (20, 20),
				text: format!("BinkyBox {}", version::VERSION).to_owned(),
				..Default::default()
			},
		);
		let lbl_1 = gui::Label::new(
			&wnd,
			gui::LabelOpts {
				position: (20, 40),
				text: "© 2023 Nadim Kobeissi".to_owned(),
				..Default::default()
			},
		);
		let btn = gui::Button::new(
			&wnd,
			gui::ButtonOpts {
				position: (20, 70),
				text: "Visit on GitHub".to_owned(),
				..Default::default()
			},
		);
		let new_self = Self {
			wnd,
			lbl_0,
			lbl_1,
			btn,
		};
		new_self.events();
		new_self
	}

	fn events(&self) {
		self.btn.on().bn_clicked(move || -> w::AnyResult<()> {
			open::that("https://github.com/nadimkobeissi/binkybox").unwrap();
			Ok(())
		});
	}
}

fn show_gui() -> w::AnyResult<i32> {
	MainWindow::new().run().map_err(|err| err.into())
}

pub fn init() {
	if let Err(e) = show_gui() {
		w::task_dlg::error(&w::HWND::NULL, "Unhandled error", None, &e.to_string())
			.unwrap();
	}
}
