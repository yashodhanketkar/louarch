package audioswitcher

import (
	"encoding/json"
	"log"
	"strings"

	"github.com/yashodhanketkar/arch/src/utils"
)

func listSources() ([]string, string) {
	sourceJSON := utils.CmdRunner("pactl", "--format=json", "list", "sources")

	var sourceList = make([]string, 0)
	var sources []Device

	if err := json.Unmarshal([]byte(sourceJSON), &sources); err != nil {
		log.Fatalf("failed to parse sources json: %v", err)
	}

	for _, source := range sources {
		name := strings.TrimSpace(source.Name)
		sourceList = append(sourceList, name)
	}

	defaultSource := strings.TrimSpace(utils.CmdRunner("pactl", "get-default-source"))
	return sourceList, defaultSource
}

func selectSource() string {
	sourceList, defaultSource := listSources()
	selectedSource, ok := utils.WofiPrompt("Select audio input: ", sourceList...)

	if !ok || selectedSource == defaultSource {
		log.Fatalf("Error selecting new source. Default source %s unchanged", defaultSource)
	}

	return selectedSource
}
