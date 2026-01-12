package browser

const (
	ShortDesc = "Browsing shortcuts"
	LongDesc  = `Provides browsing shortcuts to the user.

This command will provide user abiliy to search or visit websites with
help of wofi and firefox browser. This command have two modes of
operation:

1. Search mode: This mode will prompt user via wofi to search for a
term. Based on the search term, it will open the search result in the
firefox browser. Default search engine is duckduckgo.

2. Bookmarks mode: This mode will prompt user via wofi to select
Bookmarks. Based on the selected bookmark, it will open the bookmark in
the firefox browser. This booksmarks can be modified by the user inside
wofi prompt using 'option' item. This will allow user to add or remove
bookmarks.`
)
