package audioswitcher

import (
	"log"

	"github.com/yashodhanketkar/arch/src/utils"
)

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
