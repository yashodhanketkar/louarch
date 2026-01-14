package hyprmodes

import (
	"github.com/yashodhanketkar/louarch/src/internal"
)

type OsMode = string

const (
	GameMode  OsMode = "game"
	NightMode OsMode = "night"
)

var Modes = internal.New(map[OsMode]func(ctx *internal.Context){
	GameMode:  func(ctx *internal.Context) { toggleGM() },
	NightMode: func(ctx *internal.Context) { toggleNM() },
})
