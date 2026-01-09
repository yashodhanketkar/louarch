package cmd

import (
	"github.com/spf13/cobra"
	"github.com/yashodhanketkar/louarch/src/browser"
	"github.com/yashodhanketkar/louarch/src/utils"
)

var browserMode browser.BrowsingMode

var browserCmd = &cobra.Command{
	Use:   "browser",
	Short: browser.ShortDesc,
	Long:  browser.LongDesc,
	PreRun: func(cmd *cobra.Command, args []string) {
		utils.EarlyExit(utils.Wofi)
	},
	RunE: func(cmd *cobra.Command, args []string) error {
		return browser.Modes.Call(browserMode)
	},
}

func init() {
	rootCmd.AddCommand(browserCmd)
	browserCmd.GroupID = "productivity"

	browserCmd.Flags().StringVarP(
		&browserMode, "mode", "m", "search",
		"options: "+browser.Modes.Available(),
	)
	browserCmd.RegisterFlagCompletionFunc(
		"mode",
		func(cmd *cobra.Command, args []string, toComplete string) ([]string, cobra.ShellCompDirective) {
			return browser.Modes.Values(), cobra.ShellCompDirectiveNoFileComp
		},
	)
}
