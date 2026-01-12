package hyprmodes

import (
	"github.com/yashodhanketkar/louarch/src/utils"
)

type OsMode = string

const (
	GameMode  OsMode = "game"
	NightMode OsMode = "night"
)

var Modes = utils.New(map[OsMode]func(){
	GameMode:  toggleGM,
	NightMode: toggleNM,
})
