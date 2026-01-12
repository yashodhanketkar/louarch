package cmd

import (
	"github.com/spf13/cobra"
	"github.com/yashodhanketkar/louarch/src/hyprland/hyprpaper"
	"github.com/yashodhanketkar/louarch/src/utils"
)

var wallpaperMode hyprpaper.WallpaperMode

var wallpaperCmd = &cobra.Command{
	Use:   "wallpaper",
	Short: hyprpaper.ShortDesc,
	Long:  hyprpaper.LongDesc,
	PreRun: func(cmd *cobra.Command, args []string) {
		utils.EarlyExit(utils.Wofi)
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
}
