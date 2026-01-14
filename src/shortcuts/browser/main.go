package browser

import (
	"github.com/yashodhanketkar/louarch/src/internal"
)

type BrowsingMode = string

const (
	SearchMode    BrowsingMode = "search"
	BookmarksMode BrowsingMode = "bookmarks"
)

var Modes = internal.New(map[BrowsingMode]func(ctx *internal.Context){
	SearchMode:    func(ctx *internal.Context) { search() },
	BookmarksMode: func(ctx *internal.Context) { selectOptions() },
})
