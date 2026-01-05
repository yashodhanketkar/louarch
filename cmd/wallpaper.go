/*
Copyright © 2026 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"github.com/spf13/cobra"
	"github.com/yashodhanketkar/arch/src/hyprpaper"
)

// wallpaperCmd represents the wallpaper command
var wallpaperCmd = &cobra.Command{
	Use:   "wallpaper",
	Short: "Wallpaper switcher",
	Long:  `Update wallpaper and theme for personal linux configuration`,
	Run: func(cmd *cobra.Command, args []string) {
		hyprpaper.WallSwitcher()
	},
}

func init() {
	rootCmd.AddCommand(wallpaperCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// wallpaperCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// wallpaperCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
