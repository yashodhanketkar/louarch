package cmd

import (
	"log"

	"github.com/spf13/cobra"
	"github.com/yashodhanketkar/louarch/src/audioswitcher"
)

// wallpaperCmd represents the wallpaper command
var audioCmd = &cobra.Command{
	Use:   "audio switcher",
	Short: "Select audio devices",
	Long: `Select audio devices for your system

This command will prompt user via wofi to select audio device based on
currently available sources or sinks.`,
	Run: func(cmd *cobra.Command, args []string) {
		audioswitcher.AudioSwitcher(parseAudioTypeFlag(cmd))
	},
}

func parseAudioTypeFlag(cmd *cobra.Command) string {
	audioType, err := cmd.Flags().GetString("type")
	if err != nil {
		log.Fatal(err.Error())
	}
	return audioType
}

func init() {
	rootCmd.AddCommand(audioCmd)
	audioCmd.Flags().StringP("type", "t", "sink", "type of audio switcher")
}
