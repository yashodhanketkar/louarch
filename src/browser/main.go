package browser

import (
	"log"
)

func modeSelector(mode string) {
	switch mode {
	case "search":
		search()
	case "bookmarks":
		selectOptions()
	default:
		log.Fatal("invalid mode")
	}
}

func Browser(mode string) {
	modeSelector(mode)
}
