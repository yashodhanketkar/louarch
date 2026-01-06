package browser

import (
	"log"

	"github.com/yashodhanketkar/arch/src/utils"
)

func getSearchItem() string {
	getSearchTerm, ok := utils.WofiPrompt("Search for: ")
	if !ok {
		log.Fatalf("Invalid search item")
	}
	return getSearchTerm
}

func search() {
	searchTerm := getSearchItem()
	utils.CmdRunner(BROWSER, SEARCHENGINE+searchTerm)
}
