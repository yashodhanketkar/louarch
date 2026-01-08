package cmd

import (
	"github.com/spf13/cobra"
	"github.com/yashodhanketkar/louarch/src/hyprland"
)

var osmodeCmd = &cobra.Command{
	Use:   "osmode",
	Short: "Set operating system modes",
	Long: `Set operating system modes

This command will allow user to toggle between system modes.
Currently, there are two modes available:

1. Game mode: This mode will disable bluring, animation, etc. to give user
more performance.
2. Night mode: This mode will enable night light/bluelight filter to give
user more comfortable viewing.`,
	Args: cobra.MatchAll(
		cobra.ExactArgs(1),
		cobra.OnlyValidArgs,
	),
	ValidArgs: hyprland.Modes.Values(),
	RunE: func(cmd *cobra.Command, args []string) error {
		return hyprland.Modes.Call(hyprland.Mode(args[0]))
	},
}

func init() {
	rootCmd.AddCommand(osmodeCmd)
	osmodeCmd.GroupID = "hyprland"
}
