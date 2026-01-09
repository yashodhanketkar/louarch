package hyprland

import (
	"github.com/yashodhanketkar/louarch/src/utils"
)

type Mode = string

const (
	GameMode  Mode = "game"
	NightMode Mode = "night"
)

var Modes = utils.New(map[Mode]func(){
	GameMode:  toggleGM,
	NightMode: toggleNM,
})
