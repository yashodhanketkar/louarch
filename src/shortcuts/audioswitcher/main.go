package audioswitcher

import (
	"github.com/yashodhanketkar/louarch/src/internal"
	"github.com/yashodhanketkar/louarch/src/utils"
)

type AudioDevices = utils.AudioDevices
type AudioType = string

const (
	AudioSink   AudioType = "sink"
	AudioSource AudioType = "source"
)

var Modes = internal.New(map[AudioType]func(ctx *internal.Context){
	AudioSink:   func(c *internal.Context) { setSink() },
	AudioSource: func(c *internal.Context) { setSource() },
})
