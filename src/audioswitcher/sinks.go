package audioswitcher

import (
	"encoding/json"
	"log"
	"strings"

	"github.com/yashodhanketkar/arch/src/utils"
)

func listSinks() ([]string, string) {
	sinksJson := utils.CmdRunner("pactl", "--format=json", "list", "sinks")

	var sinkList = make([]string, 0)
	var sinks []Device

	if err := json.Unmarshal([]byte(sinksJson), &sinks); err != nil {
		log.Fatalf("failed to parse sinks json: %v", err)
	}

	for _, sink := range sinks {
		name := strings.TrimSpace(sink.Name)
		sinkList = append(sinkList, name)
	}

	defaultSink := strings.TrimSpace(utils.CmdRunner("pactl", "get-default-sink"))

	return sinkList, defaultSink
}

func selectSink() string {
	sinkList, defaultSink := listSinks()
	selectedSink, ok := utils.WofiPrompt("Select audio output: ", sinkList...)

	if !ok || selectedSink == defaultSink {
		log.Fatalf("Error selecting new sink. Default sink %s unchanged", defaultSink)
	}

	return selectedSink
}
