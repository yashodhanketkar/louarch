Louarch
=======

Faster, safer and efficient rust port of louarch.

Currently, this project is in development stage. Follwing features are planned:

-	[X] Audio switcher
-	[X] Browser launcher
-	[X] Completions generator
-	[X] Config handler
-	[X] Network switcher
-	[X] OS Mode switcher
-	[X] Tmux handler
-	[X] Wallpaper switcher

Table of Contents
-----------------

-	[Usage](#usage)
	-	[Audio](#audio)
	-	[Browser](#browser)
	-	[Completions](#completions)
	-	[Config](#config)
	-	[Network](#network)
	-	[OS Mode](#os-mode)
	-	[Tmux](#tmux)
	-	[Wallpaper](#wallpaper)
-	[Configuration](#configuration)
	-	[Requirements](#requirements)
	-	[Install](#install)
	-	[Running](#running)
	-	[Uninstall](#uninstall)
-	[LICENSE](#license)

Usage
-----

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

### Completions

Completions for bash and zsh

This command will allow user to generate completions for bash and zsh. Currently, two shells are supported.

1.	Bash: This will generate completions for bash.
2.	Zsh: This will generate completions for zsh.

cmd: `completions [args]`

| Args | Description         |
|------|---------------------|
| bash | generate bash comps |
| zsh  | generate zsh comps  |

### Config

Config handler

This command allows user to:

1.	View currently laoded config
2.	Edit config file

cmd `config [args]`

| Args     | Description            |
|----------|------------------------|
| view     | view current config    |
| edit     | edit config file       |
| keybinds | view hyprland keybinds |

### Network

Toggle connected devices/networks

This command will allow user to handle devices/networks connections. Currently supports WiFi and bluetooth.

cmd: `network [args] [sub-args]`

| Args      | Sub        | Description                        |
|-----------|------------|------------------------------------|
| wifi      | connect    | connect to a wifi network          |
| wifi      | disconnect | disconnect from a wifi network     |
| bluetooth | connect    | connect to a bluetooth device      |
| bluetooth | disconnect | disconnect from a bluetooth device |

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

### Tmux

Connect or kill tmux sessions

This command will allow user to connect (if exists or spawns) or kill tmux sessions.

cmd: `tmux [args]`

| Args   | Description              |
|--------|--------------------------|
| attach | attach to a tmux session |
| kill   | kill a tmux session      |

### Wallpaper

Update wallpaper and theme for personal linux configuration.

This command will prompt user via rofi to select wallpaper based on the current monitors. Based on selected wallpapers, it will generate a theme pallete with help of wallust. Finally, it will apply theme colors to the UI tools such as waybar, swaync, etc. Configuration

cmd: `wallpaper`

| Args   | Description                 |
|--------|-----------------------------|
| select | Select and apply wallpapers |
| random | Apply a random wallpaper    |

Configuration
-------------

### Requirements

-	Rustup/Cargo 1.90 or higher
-	Hyprland 0.55.0 or higher
-	Rofi 2.0.0 or higher
-	wallust 3.4.0 or higher
-	UPX 5.1.1 or higher (Optional) (For compressing binary)
-	GNU Make 4.4.1 (Optional) (For automated build process)

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
./target/release/louarch
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

This project is licensed under the [GNU GPLv3.0 License](./LICENSE) - see the [license](./LICENSE) file for details.
