package cmd

import (
	"github.com/spf13/cobra"
	"github.com/yashodhanketkar/louarch/src/audioswitcher"
	"github.com/yashodhanketkar/louarch/src/utils"
)

var mode audioswitcher.AudioType

var audioCmd = &cobra.Command{
	Use:   "audio",
	Short: "Select audio devices",
	Long: `Select audio devices for your system

This command will prompt user via wofi to select audio device based on
currently available sources or sinks.`,
	PreRun: func(cmd *cobra.Command, args []string) {
		utils.EarlyExit(utils.Wofi)
	},
	RunE: func(cmd *cobra.Command, args []string) error {
		return audioswitcher.Modes.Call(mode)
	},
}

func init() {
	rootCmd.AddCommand(audioCmd)
	audioCmd.GroupID = "productivity"

	audioCmd.Flags().
		StringVarP(&mode, "type", "t", "sink", "mode: "+audioswitcher.Modes.Available())
}
