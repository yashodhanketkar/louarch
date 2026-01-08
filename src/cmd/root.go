package cmd

import (
	"os"

	"github.com/spf13/cobra"
)

// rootCmd represents the base command when called without any subcommands
var rootCmd = &cobra.Command{
	Use:   "louarch",
	Short: "Personalization tool for personal linux setup",
	Long: `This application provides a set of perosnalization tools for Arch Linux.

This tool is written for personal setup which consists of hypraland
winodws manager (powered by wayland). It provides following tools:

1. Audio - Switches between audio devices. (both Sink and Source)
2. Wallpaper - Changes wallpaper and apply new theme generated from wallpaper.
3. Browser - Bookmark and search with wofi and browser (default: firefox).
4. Osmode - Toggle between diffrent system modes.`,
}

func Execute() {
	err := rootCmd.Execute()
	if err != nil {
		os.Exit(1)
	}
}

func init() {
	rootCmd.Flags().BoolP("root", "r", false, "Help message for toggle")
	rootCmd.AddGroup(&cobra.Group{ID: "hyprland", Title: "Hyprland managment commands"})
	rootCmd.AddGroup(&cobra.Group{ID: "productivity", Title: "Productivity shortcuts"})
}
