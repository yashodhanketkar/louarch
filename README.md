Louarch
=======

Faster, safer and efficient rust port of louarch.

Currently, this project is in development stage. Follwing features are planned:

-	[X] Wallpaper switcher
-	[X] OS Mode switcher
-	[X] Audio switcher
-	[X] Browser launcher

Table of Contents
-----------------

-	[Usage](#usage)
	-	[Wallpaper](#wallpaper)
	-	[OS Mode](#os-mode)
-	[Configuration](#configuration)
	-	[Requirements](#requirements)
	-	[Install](#install)
	-	[Running](#running)
	-	[Uninstall](#uninstall)
-	[LICENSE](#license)

Usage
-----

### Wallpaper

Update wallpaper and theme for personal linux configuration.

This command will prompt user via rofi to select wallpaper based on the current monitors. Based on selected wallpapers, it will generate a theme pallete with help of wallust. Finally, it will apply theme colors to the UI tools such as waybar, swaync, etc. Configuration

cmd: `wallpaper`

| Args   | Description                 |
|--------|-----------------------------|
| select | Select and apply wallpapers |
| random | Apply a random wallpaper    |

### OS Mode

Set operating system modes

This command will allow user to toggle between system modes. Currently, there are two modes available:

1.	Game mode: This mode will disable bluring, animation, etc. to give user more performance.
2.	Night mode: This mode will enable night light/bluelight filter to give user more comfortable viewing.

cmd: `osmode [args]`

| Args  | Description                                |
|-------|--------------------------------------------|
| game  | toggle gaming mode                         |
| night | toggle night-time or bluelight filter mode |

### Audio

Switch audio devices

This command will allow user to switch between audio devices. Currently, two options are available:

1.	Sink: This will allow user to set default audio sink. (Output device)
2.	Source: This will allow user to set default audio source. (Input device)

cmd: `audio [args]`

| Args   | Description         |
|--------|---------------------|
| sink   | switch audio sink   |
| source | switch audio source |

### Browser

Launch browser

This command will allow user to launch a browser. Currently, two options are available:

1.	Search: This will allow user to search for a term.
2.	Browse: This will allow user to open a URL.

cmd: `browser [args]`

| Args   | Description       |
|--------|-------------------|
| search | search for a term |
| browse | open a URL        |

Configuration
-------------

### Requirements

-	Rustup/Cargo 1.90 or higher
-	Rofi 2.0.0 or higher
-	wallust 3.4.0 or higher
-	GNU Make 4.4.1 (Optional)

### Install

Make sure you have above tools installed before proceeding with installation.

```bash
# with make installed 
make release

# without make installed
cargo build --release
```

### Running

```sh
# with make installed
make run

# without make installed
./target/release/louarchrs
```

### Uninstall

```sh
# with make installed
make clean

# without make installed
cargo clean
```

LICENSE
-------

[MIT](./LICENSE)

Copyright (c) 2026 Yashodhan Ketkar
