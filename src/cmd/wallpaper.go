package cmd

import (
	"github.com/spf13/cobra"
	"github.com/yashodhanketkar/louarch/src/hyprland/hyprpaper"
	"github.com/yashodhanketkar/louarch/src/internal"
)

var wallpaperMode hyprpaper.WallpaperMode
var path string

var wallpaperCmd = &cobra.Command{
	Use:   "wallpaper",
	Short: hyprpaper.ShortDesc,
	Long:  hyprpaper.LongDesc,
	PreRunE: func(cmd *cobra.Command, args []string) error {
		internal.EarlyExit(internal.Wofi)

		// handle required flags for [set] mod
		if wallpaperMode == "set" && path == "" {
			return cobra.MarkFlagRequired(cmd.Flags(), "path")
		}
		hyprpaper.Modes.SetContext(&internal.Context{Path: path})

		return nil
	},
	RunE: func(cmd *cobra.Command, args []string) error {
		return hyprpaper.Modes.Call(wallpaperMode)
	},
}

func init() {
	rootCmd.AddCommand(wallpaperCmd)
	wallpaperCmd.GroupID = "Hyprland"

	wallpaperCmd.Flags().StringVarP(
		&wallpaperMode, "mode", "m", "apply",
		"options: "+hyprpaper.Modes.Available(),
	)

	wallpaperCmd.RegisterFlagCompletionFunc(
		"mode",
		func(cmd *cobra.Command, args []string, toComplete string) ([]string, cobra.ShellCompDirective) {
			return hyprpaper.Modes.Values(), cobra.ShellCompDirectiveNoFileComp
		},
	)

	wallpaperCmd.Flags().StringVarP(&path, "path", "p", "", "Path to wallpaper")
}
