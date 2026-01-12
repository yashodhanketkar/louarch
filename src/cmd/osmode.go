package cmd

import (
	"github.com/spf13/cobra"
	"github.com/yashodhanketkar/louarch/src/hyprland/hyprmodes"
)

var osmodeCmd = &cobra.Command{
	Use:   "osmode",
	Short: hyprmodes.ShortDesc,
	Long:  hyprmodes.LongDesc,
	Args: cobra.MatchAll(
		cobra.ExactArgs(1),
		cobra.OnlyValidArgs,
	),
	ValidArgs: hyprmodes.Modes.Values(),
	RunE: func(cmd *cobra.Command, args []string) error {
		return hyprmodes.Modes.Call(hyprmodes.OsMode(args[0]))
	},
}

func init() {
	rootCmd.AddCommand(osmodeCmd)
	osmodeCmd.GroupID = "Hyprland"
}
