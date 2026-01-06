package browser

import (
	"fmt"

	"github.com/yashodhanketkar/arch/src/utils"
)

const (
	BOOKMARKS_FILE = utils.BookmarksFile
	BROWSER        = utils.Browser
	SEARCHENGINE   = utils.SearchEngine
)

func modeSelector(mode string) {
	switch mode {
	case "search":
		search()
	case "bookmarks":
		selectOptions()
	default:
		fmt.Println("invalid mode")
	}
}

func Browser(mode string) {
	modeSelector(mode)
}
