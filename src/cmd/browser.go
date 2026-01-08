package cmd

import (
	"log"

	"github.com/spf13/cobra"
	"github.com/yashodhanketkar/louarch/src/browser"
	"github.com/yashodhanketkar/louarch/src/utils"
)

var browserCmd = &cobra.Command{
	Use:   "browser",
	Short: "Browsing shortcuts",
	Long: `Provides browsing shortcuts to the user.

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
bookmarks.`,
	PreRun: func(cmd *cobra.Command, args []string) {
		utils.EarlyExit(utils.Wofi)
	},
	Run: func(cmd *cobra.Command, args []string) {
		browser.Browser(parseBrowsingModeFlag(cmd))
	},
}

func parseBrowsingModeFlag(cmd *cobra.Command) string {
	browsingMode, err := cmd.Flags().GetString("mode")
	if err != nil {
		log.Fatal(err)
	}
	return browsingMode
}

func init() {
	rootCmd.AddCommand(browserCmd)
	browserCmd.Flags().StringP("mode", "m", "search", "select serach or browsing mode")
	browserCmd.GroupID = "productivity"
}
