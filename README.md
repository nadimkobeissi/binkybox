# BinkyBox

![Binky](icons/binky.ico)

## The Problem

Windows 10 [introduced virtual desktop support in 2015](https://www.pcworld.com/article/1936035/virtual-desktops-more-space-and-order-with-windows-10-and-11.html). This led to the obsolescence of third-party virtual desktop software, which is a problem, since Windows's native virtual desktop support lacks two very important features:

1. Ability to hop from one desktop to another directly without going through intermediate desktops in between (for example, going straight from desktop 1 to desktop 3) via keyboard shortcuts.
2. Ability to see which desktop you're on right now thanks to a system tray icon.

Indeed, Windows 11 only comes with two shortcuts for navigating virtual desktops:

- _Win+Ctrl+🠊_: Switch a desktop to the right.
- _Win+Ctrl+🠈_: Switch a desktop to the left.

Not only is this ridiculous, but there's no way to add additional shortcuts without writing code that calls Windows's exposed COM API directly.

## The Solution!

**BinkyBox** lets you _hop_ from one desktop to another via keyboard shortcuts and adds a useful system tray icon so you know which desktop you're on. The logo is a cute bunny because bunnies hop. Like you're hopping from one desktop to another. Using keyboard shortcuts. Let's all hop together using BinkyBox.

A "binky" is a word for [a kind of happy hop that bunnies do](https://be.chewy.com/is-this-normal-why-do-rabbits-binky/).

### Incredible Features:

- Hop from one virtual desktop to another _directly_ using incredible keyboard shortcuts (_Ctrl+Alt+1_, _Ctrl+Alt+2_, etc.)
- See which virtual desktop you're on right now via a system tray icon.
- Written in RUST! WOW! NERD POINTS! RUST!!!! VERY IMPORTANT THAT THIS SOFTWARE BE WRITTEN IN RUST. LOOK AT HOW SMART THIS SOFTWARE IS. RUST! THE ONLY LANGUAGE THAT PEOPLE ARE ALLOWED TO USE OTHERWISE THEY SUCK AT COMPUTER PROGRAMMING. RUST!!! FERRIS THE FERRIS WHEEL CRAB!! RUST!!!!!!!!!!!!!!!

I may add other features later.

## Tested Platforms

| Windows            | Tested     |
|--------------------|------------|
| Windows 11 23H2    | ✔️         |
| Windows 11 (Older) | ✔️         |
| Windows 10         | ❌         |

## License and Author

License: MIT. Author: [Nadim Kobeissi](https://nadim.computer)
