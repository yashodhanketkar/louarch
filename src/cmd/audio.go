package cmd

import (
	"github.com/spf13/cobra"
	"github.com/yashodhanketkar/louarch/src/internal"
	"github.com/yashodhanketkar/louarch/src/shortcuts/audioswitcher"
)

var audioMode audioswitcher.AudioType

var audioCmd = &cobra.Command{
	Use:   "audio",
	Short: audioswitcher.ShortDesc,
	Long:  audioswitcher.LongDesc,
	PreRun: func(cmd *cobra.Command, args []string) {
		internal.EarlyExit(internal.Wofi)
	},
	RunE: func(cmd *cobra.Command, args []string) error {
		return audioswitcher.Modes.Call(audioMode)
	},
}

func init() {
	rootCmd.AddCommand(audioCmd)
	audioCmd.GroupID = "Shortcuts"

	audioCmd.Flags().StringVarP(
		&audioMode, "type", "t", "sink",
		"mode: "+audioswitcher.Modes.Available(),
	)
	audioCmd.RegisterFlagCompletionFunc(
		"type",
		func(cmd *cobra.Command, args []string, toComplete string) ([]string, cobra.ShellCompDirective) {
			return audioswitcher.Modes.Values(), cobra.ShellCompDirectiveNoFileComp
		},
	)
}
