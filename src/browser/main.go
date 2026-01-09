package browser

import (
	"github.com/yashodhanketkar/louarch/src/utils"
)

type BrowsingMode = string

const (
	SearchMode    BrowsingMode = "search"
	BookmarksMode BrowsingMode = "bookmarks"
)

var Modes = utils.New(map[BrowsingMode]func(){
	SearchMode:    search,
	BookmarksMode: selectOptions,
})
