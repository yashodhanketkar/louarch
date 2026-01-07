Louarch
=======

Rewrite of bash script with golang for better debugging and single binary.

Table of Contents
-----------------

-	[Usage](#usage)
	-	[Audio](#audio)
	-	[Wallpaper](#wallpaper)
	-	[Browsing](#browsing)
-	[Configuration](#configuration)
	-	[Requirements](#requirements)
	-	[Installation](#installation)
	-	[Uninstallation](#uninstallation)
-	[LICENSE](#license)

Usage
-----

### Audio

*Select audio devices*

Select audio devices for your system This command will prompt user via wofi to select audio device based on currently available sources or sinks.

cmd: `audio`

| Flags            | Description            | Values       |
|------------------|------------------------|--------------|
| `-t` or `--type` | type of audio switcher | sink, source |

### Browser

*Browsing shortcuts*

This command will provide user abiliy to search or visit websites with help of wofi and firefox browser. This command have two modes of operation:

1.	Search mode: This mode will prompt user via wofi to search for a term. Based on the search term, it will open the search result in the firefox browser. Default search engine is duckduckgo.

2.	Bookmarks mode: This mode will prompt user via wofi to select Bookmarks. Based on the selected bookmark, it will open the bookmark in the firefox browser. This booksmarks can be modified by the user inside wofi prompt using 'option' item. This will allow user to add or remove bookmarks.

cmd: `browser`

| Flags            | Description      | Values            |
|------------------|------------------|-------------------|
| `-m` or `--mode` | mode of browsing | search, bookmarks |

### Wallpaper

*Wallpaper switcher*

Update wallpaper and theme for personal linux configuration.

This command will prompt user via wofi to select wallpaper based on the current monitors. Based on selected wallpapers, it will generate a theme pallete with help of wallust. Finally, it will apply theme colors to the UI tools such as waybar, sway-nc, etc. Configuration

cmd: `wallpaper`

Configuration
-------------

### Requirements

-	Go 1.23 or higher
-	Wofi 1.5.1 or higher
-	wallust 3.4.0 or higher
-	GNU Make 4.4.1 (Optional)

### Installation

Make sure you have above tools installed before proceeding with installation.

```bash
# with make installed 
make install

# without make installed
chmod a+x ./install.sh
./install.sh
```

### Uninstallation

```sh
# with make installed
make uninstall

# without make installed
chmod a+x ./uninstall.sh
./uninstall.sh
```

LICENSE
-------

[MIT](./LICENSE)

Copyright (c) 2026 Yashodhan Ketkar
