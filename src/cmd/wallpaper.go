package cmd

import (
	"github.com/spf13/cobra"
	"github.com/yashodhanketkar/arch/src/hyprpaper"
)

// wallpaperCmd represents the wallpaper command
var wallpaperCmd = &cobra.Command{
	Use:   "wallpaper",
	Short: "Wallpaper switcher",
	Long: `Update wallpaper and theme for personal linux configuration.

This command will prompt user via wofi to select wallpaper based on the
current monitors. Based on selected wallpapers, it will generate a theme
pallete with help of wallust. Finally, it will apply theme colors to the
UI tools such as waybar, sway-nc, etc.`,
	Run: func(cmd *cobra.Command, args []string) {
		hyprpaper.WallSwitcher()
	},
}

func init() {
	rootCmd.AddCommand(wallpaperCmd)
}
