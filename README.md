# BinkyBox

![Binky](icons/binky.ico)

## Problem

Windows 10 [introduced virtual desktop support in 2015](https://www.pcworld.com/article/1936035/virtual-desktops-more-space-and-order-with-windows-10-and-11.html). This led to the obsolescence of third-party virtual desktop software, which is a problem, since Windows's native virtual desktop support lacks two very important features:

1. Ability to hop from one desktop to another directly without going through intermediate desktops in between (for example, going straight from desktop 1 to desktop 3) via keyboard shortcuts.
2. Ability to see which desktop you're on right now thanks to a system tray icon.

Indeed, Windows 11 only comes with two shortcuts for navigating virtual desktops:

- _Win+Ctrl+🠊_: Switch a desktop to the right.
- _Win+Ctrl+🠈_: Switch a desktop to the left.

Not only is this ridiculous, but there's no way to add additional shortcuts without writing code that calls Windows's exposed COM API directly.

## Solution

**BinkyBox** lets you _hop_ from one desktop to another via keyboard shortcuts and adds a useful system tray icon so you know which desktop you're on. The logo is a cute bunny because bunnies hop. Like you're hopping from one desktop to another. Using keyboard shortcuts. Let's all hop together using BinkyBox.

A "binky" is a word for [a kind of happy hop that bunnies do](https://www.youtube.com/watch?v=y0ivoIr_xnQ).

### Incredible Features

- Hop from one virtual desktop to another _directly_ using incredible keyboard shortcuts.
- See which virtual desktop you're on right now via a system tray icon.
- Written in Rust, leading to very low (almost nonexistent) background resource usage.

I may add other features later.

### Feature Requests Welcome

I use BinkyBox daily at work. If you use it a lot too, please don't hesitate to ask for features by opening a GitHub issue. I will likely maintain this software for as long as I continue to use Windows.

### Tested Platforms

| Windows            | Tested     |
|--------------------|------------|
| Windows 11 23H2    | ✔️         |
| Windows 11 (Older) | ✔️         |
| Windows 10         | ❌         |

## Build and Use

### Download the App

You can get pre-build binaries from the [GitHub Releases](https://github.com/nadimkobeissi/binkybox/releases) page.

### Compile it Yourself

You will need to have [Rust](https://rustup.rs) installed.

```bash
git clone https://github.com/nadimkobeissi/binkybox
cd binkybox
cargo build --release
```

The `.exe` will be in `target/release`.

## Setting Keyboard Shortcuts

Right click the tray icon and open _Settings_ in order to set your own keyboard shortcuts. BinkyBox keyboard shortcuts follow this layout: `MODIFIER+OPTIONAL_MODIFIER+OPTIONAL_MODIFIER+OPTIONAL_MODIFIER+ALPHANUMERIC_CHAR_OR_FKEY`.

The first modifier is required, and ending the sequence with an alphanumeric character is also required.

- **Supported Modifiers**: `LCTRL`, `RCTRL`, `LALT`, `RALT`, `LWIN`, `RWIN`, `LSHIFT`, `RSHIFT` and F keys (`F1` to `F12`).
- **Example Keyboard Shortcuts**: `LCTRL+LALT+1`, `LWIN+LSHIFT+1`

BinkyBox will reject keyboard shortcuts not specified using the layout above, so make sure you enter your keyboard shortcuts correctly. Keyboard shortcuts (and other settings) will be saved in a `binkybox.config.json` file located in the same directory as the BinkyBox executable.

## Adding to Windows Startup

You can make BinkyBox start automatically on Windows startup when you log in:

1. Create a shortcut for `binkybox.exe` by right-clicking it.
2. Press _Win+R_ to open Run, and type in `shell:common startup`.
3. Paste the BinkyBox shortcut you created in Step 1 inside the resulting Startup folder.

## Programs with Elevated Privileges

Hotkeys may not be captured or processed if the currently focused program is running with elevated privileges (admin rights) while BinkyBox is running with normal user privileges.
A suggested workaround is to run BinkyBox with elevated privileges (as admin) as well.

## License and Author

License: MIT. Author: [Nadim Kobeissi](https://nadim.computer)
