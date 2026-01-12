package audioswitcher

import (
	"github.com/yashodhanketkar/louarch/src/utils"
)

type AudioDevices = utils.AudioDevices
type AudioType = string

const (
	AudioSink   AudioType = "sink"
	AudioSource AudioType = "source"
)

var Modes = utils.New(map[AudioType]func(){
	AudioSink:   setSink,
	AudioSource: setSource,
})
