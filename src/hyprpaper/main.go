package hyprpaper

import (
	"github.com/yashodhanketkar/louarch/src/utils"
)

type Monitor = utils.Monitor

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
