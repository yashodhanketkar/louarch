package utils

import (
	"os"
	"regexp"
)

var (
	configFile = os.ExpandEnv("$HOME/.config/louarch/config.json")
	AppConfig  = Config{}
)

func handleEnv(config *Config) {
	re := regexp.MustCompile("$HOME|~")

	// handle both $HOME and ~ paths
	config.BookmarksFile = re.ReplaceAllString(config.BookmarksFile, os.Getenv("HOME"))
	config.ConfigPath = re.ReplaceAllString(config.ConfigPath, os.Getenv("HOME"))
	config.WallpaperDir = re.ReplaceAllString(config.WallpaperDir, os.Getenv("HOME"))
}

func Configure() {
	readConfig(os.ExpandEnv("$HOME/.local/share/louarch/config.json"))

	// if user config exists, replace default values with user values
	if FileExists(configFile) {
		readConfig(configFile)
	}

	handleEnv(&AppConfig)
}
