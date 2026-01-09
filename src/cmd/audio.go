package cmd

import (
	"github.com/spf13/cobra"
	"github.com/yashodhanketkar/louarch/src/audioswitcher"
	"github.com/yashodhanketkar/louarch/src/utils"
)

var audioMode audioswitcher.AudioType

var audioCmd = &cobra.Command{
	Use:   "audio",
	Short: audioswitcher.ShortDesc,
	Long:  audioswitcher.LongDesc,
	PreRun: func(cmd *cobra.Command, args []string) {
		utils.EarlyExit(utils.Wofi)
	},
	RunE: func(cmd *cobra.Command, args []string) error {
		return audioswitcher.Modes.Call(audioMode)
	},
}

func init() {
	rootCmd.AddCommand(audioCmd)
	audioCmd.GroupID = "productivity"

	audioCmd.Flags().StringVarP(
		&audioMode, "type", "t", "sink",
		"mode: "+audioswitcher.Modes.Available(),
	)
}
