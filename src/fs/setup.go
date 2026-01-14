package fs

import (
	"os"
	"regexp"
)

func handleEnv(config *Config) {
	re := regexp.MustCompile("$HOME|~")

	// handle both $HOME and ~ paths
	config.BookmarksFile = re.ReplaceAllString(config.BookmarksFile, os.Getenv("HOME"))
	config.ConfigPath = re.ReplaceAllString(config.ConfigPath, os.Getenv("HOME"))
	config.WallpaperDir = re.ReplaceAllString(config.WallpaperDir, os.Getenv("HOME"))
}

func Configure() {
	ReadConfig(os.ExpandEnv("$HOME/.local/share/louarch/config.json"), &AppConfig)

	// if user config exists, replace default values with user values
	if FileExists(configFile) {
		ReadConfig(configFile, &AppConfig)
	}

	handleEnv(&AppConfig)
}
