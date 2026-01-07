package audioswitcher

import (
	"log"

	"github.com/yashodhanketkar/louarch/src/utils"
)

type AudioDevices = utils.AudioDevices

func audioSetter(audioType string) {
	switch audioType {
	case "sink":
		sinkName := selectSink()
		utils.CmdRunner("pactl", "set-default-sink", sinkName)
	case "source":
		sourceName := selectSource()
		utils.CmdRunner("pactl", "set-default-source", sourceName)
	default:
		log.Fatalf("invalid sink type %s", audioType)
	}
}

func AudioSwitcher(audioType string) {
	audioSetter(audioType)
}
