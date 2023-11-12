/* SPDX-FileCopyrightText: © 2023 Nadim Kobeissi <nadim@symbolic.software>
 * SPDX-License-Identifier: MIT */

use winsafe::{self as w, co, gui, prelude::*};

mod util;

#[derive(Clone)]
pub struct MainWindow {
	wnd: gui::WindowMain,
	tab_ctrl: gui::Tab,
}

impl MainWindow {
	pub fn new() -> Self {
		let wnd = gui::WindowMain::new(gui::WindowMainOpts {
			title: "BinkyBox Settings".to_owned(),
			class_icon: gui::Icon::Str(winsafe::WString::from_str("binky")),
			size: (300, 500),
			style: gui::WindowMainOpts::default().style | co::WS::MINIMIZEBOX,
			..Default::default()
		});

		let tab_ctrl = gui::Tab::new(
			&wnd,
			gui::TabOpts {
				position: (10, 10),
				size: (280, 480),
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

fn run_app() -> w::AnyResult<i32> {
	MainWindow::new().run().map_err(|err| err.into())
}

#[derive(Clone)]
pub struct TabContainerKeyboardShortcuts {
	wnd: gui::WindowControl,
	lbl_0: gui::Label,
	txt_0: gui::Edit,
	btn_0: gui::Button,
	
	lbl_1: gui::Label,
	txt_1: gui::Edit,
	btn_1: gui::Button,

	lbl_2: gui::Label,
	txt_2: gui::Edit,
	btn_2: gui::Button,

	lbl_3: gui::Label,
	txt_3: gui::Edit,
	btn_3: gui::Button,
}

impl GuiTab for TabContainerKeyboardShortcuts {
	fn as_ctrl(&self) -> &gui::WindowControl {
		&self.wnd
	}
}

impl TabContainerKeyboardShortcuts {
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
				text: "Switch to Desktop 1".to_owned(),
				..Default::default()
			},
		);

		let txt_0 = gui::Edit::new(
			&wnd,
			gui::EditOpts {
				position: (20, 40),
				width: 180,
				text: "Ctrl+Alt+1".to_owned(),
				edit_style: co::ES::READONLY,
				..Default::default()
			},
		);

		let btn_0 = gui::Button::new(
			&wnd,
			gui::ButtonOpts {
				position: (20, 70),
				text: "Set".to_owned(),
				..Default::default()
			},
		);

		let lbl_1 = gui::Label::new(
			&wnd,
			gui::LabelOpts {
				position: (20, 120),
				text: "Switch to Desktop 2".to_owned(),
				..Default::default()
			},
		);

		let txt_1 = gui::Edit::new(
			&wnd,
			gui::EditOpts {
				position: (20, 140),
				width: 180,
				text: "Ctrl+Alt+2".to_owned(),
				edit_style: co::ES::READONLY,
				..Default::default()
			},
		);

		let btn_1 = gui::Button::new(
			&wnd,
			gui::ButtonOpts {
				position: (20, 170),
				text: "Set".to_owned(),
				..Default::default()
			},
		);

		let lbl_2 = gui::Label::new(
			&wnd,
			gui::LabelOpts {
				position: (20, 220),
				text: "Switch to Desktop 3".to_owned(),
				..Default::default()
			},
		);

		let txt_2 = gui::Edit::new(
			&wnd,
			gui::EditOpts {
				position: (20, 240),
				width: 180,
				text: "Ctrl+Alt+3".to_owned(),
				edit_style: co::ES::READONLY,
				..Default::default()
			},
		);

		let btn_2 = gui::Button::new(
			&wnd,
			gui::ButtonOpts {
				position: (20, 270),
				text: "Set".to_owned(),
				..Default::default()
			},
		);

		let lbl_3 = gui::Label::new(
			&wnd,
			gui::LabelOpts {
				position: (20, 320),
				text: "Switch to Desktop 4".to_owned(),
				..Default::default()
			},
		);

		let txt_3 = gui::Edit::new(
			&wnd,
			gui::EditOpts {
				position: (20, 340),
				width: 180,
				text: "Ctrl+Alt+4".to_owned(),
				edit_style: co::ES::READONLY,
				..Default::default()
			},
		);

		let btn_3 = gui::Button::new(
			&wnd,
			gui::ButtonOpts {
				position: (20, 370),
				text: "Set".to_owned(),
				..Default::default()
			},
		);

		let new_self = Self {
			wnd,
			lbl_0, txt_0, btn_0,
			lbl_1, txt_1, btn_1,
			lbl_2, txt_2, btn_2,
			lbl_3, txt_3, btn_3,
		};
		new_self.events();
		new_self
	}

	fn events(&self) {
		let self0 = self.clone();
		let self1 = self.clone();
		let self2 = self.clone();
		let self3 = self.clone();
		self.btn_0.on().bn_clicked(move || {
			w::task_dlg::info(
				&self0.wnd.hwnd().GetParent()?,
				"Work in progress",
				None,
				"You can't set your own keyboard shortcuts yet.\nPlease use the defaults.",
			)?;
			Ok(())
		});
		self.btn_1.on().bn_clicked(move || {
			w::task_dlg::info(
				&self1.wnd.hwnd().GetParent()?,
				"Work in progress",
				None,
				"You can't set your own keyboard shortcuts yet.\nPlease use the defaults.",
			)?;
			Ok(())
		});
		self.btn_2.on().bn_clicked(move || {
			w::task_dlg::info(
				&self2.wnd.hwnd().GetParent()?,
				"Work in progress",
				None,
				"You can't set your own keyboard shortcuts yet.\nPlease use the defaults.",
			)?;
			Ok(())
		});
		self.btn_3.on().bn_clicked(move || {
			w::task_dlg::info(
				&self3.wnd.hwnd().GetParent()?,
				"Work in progress",
				None,
				"You can't set your own keyboard shortcuts yet.\nPlease use the defaults.",
			)?;
			Ok(())
		});
	}
}

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
				text: "BinkyBox 0.1.0".to_owned(),
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

		let new_self = Self { wnd, lbl_0, lbl_1, btn };
		new_self.events();
		new_self
	}

	fn events(&self) {
		self.btn.on().bn_clicked(move || -> w::AnyResult<()> {
			util::open_url("https://github.com/nadimkobeissi/binkybox");
			Ok(())
		});
	}
}

pub fn init() {
	if let Err(e) = run_app() {
		w::task_dlg::error(&w::HWND::NULL, "Unhandled error", None, &e.to_string())
			.unwrap();
	}
}
