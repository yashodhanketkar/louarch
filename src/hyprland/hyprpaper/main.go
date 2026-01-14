package hyprpaper

import (
	"github.com/yashodhanketkar/louarch/src/internal"
	"github.com/yashodhanketkar/louarch/src/utils"
)

type Monitor = utils.Monitor
type WallpaperMode = string

const (
	apply WallpaperMode = "apply"
	set   WallpaperMode = "set"
)

var Modes = internal.New(map[WallpaperMode]func(ctx *internal.Context){
	apply: func(ctx *internal.Context) { wallSwitcher() },
	set:   func(ctx *internal.Context) { setWallpaper(ctx.Path) },
})
