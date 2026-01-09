package cmd

import (
	"github.com/spf13/cobra"
	"github.com/yashodhanketkar/louarch/src/hyprland"
)

var osmodeCmd = &cobra.Command{
	Use:   "osmode",
	Short: hyprland.ShortDesc,
	Long:  hyprland.LongDesc,
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
