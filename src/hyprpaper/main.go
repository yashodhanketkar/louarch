package hyprpaper

import "github.com/yashodhanketkar/arch/src/utils"

type Monitor = utils.Monitor

const (
	WallpaperDir = utils.WallpaperDir
	ConfigPath   = utils.ConfigPath
)

func WallSwitcher() {
	monitors := listMonitors()
	wallpapers := listWallpapers()

	selected, ok := selectWallpapers(wallpapers, monitors)
	if !ok {
		return
	}

	updateConfig(monitors, selected)
	setupHyprpaper(selected[0])
}
