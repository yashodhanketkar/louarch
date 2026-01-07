package utils

type AudioDevices struct {
	Name string `json:"name"`
}

type Monitor struct {
	Name string `json:"name"`
}

type Config struct {
	Browser       string `json:"browser"`
	SearchEngine  string `json:"searchEngine"`
	BookmarksFile string `json:"bookmarksFile"`
	WofiCmd       string `json:"wofiCmd"`
	WallpaperDir  string `json:"wallpaperDir"`
	ConfigPath    string `json:"configPath"`
}
