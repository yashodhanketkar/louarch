package hyprpaper

import (
	"github.com/yashodhanketkar/louarch/src/utils"
)

type Monitor = utils.Monitor
type WallpaperMode = string

const (
	apply WallpaperMode = "apply"
)

var Modes = utils.New(map[WallpaperMode]func(){
	apply: wallSwitcher,
})
