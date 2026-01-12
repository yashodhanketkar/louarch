package browser

import (
	"log"

	"github.com/yashodhanketkar/louarch/src/utils"
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
	utils.CmdRunner(utils.AppConfig.Browser, utils.AppConfig.SearchEngine+searchTerm)
}
