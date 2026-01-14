package hyprmodes

import (
	"log"
	"os"

	"github.com/yashodhanketkar/louarch/src/fs"
	"github.com/yashodhanketkar/louarch/src/utils"
)

// using tmp file as this file is automatically deleted on login which
// works perfectly with temporary changes in hyprland instead of persistent
// file/config changes
var flagFile = os.ExpandEnv("$HOME/tmp/gamemode.txt")

func hyprlandBatch() []string {
	// performace tweaks for gamemode
	return []string{
		"keyword animations:enabled 0",
		"keyword animation borderangle,0",
		"keyword decoration:shadow:enabled 0",
		"keyword decoration:blur:enabled 0",
		"keyword decoration:rounding 0",
		"keyword general:gaps_in 0",
		"keyword general:gaps_out 0",
		"keyword general:border_size 0",
		"keyword decoration:fullscreen_opacity 1",
	}
}

func startGm() {
	cmds := hyprlandBatch()
	// apply commands to hyprland
	for _, cmd := range cmds {
		utils.CmdRunner("hyprctl", "--quiet", "--batch", cmd)
	}
	os.Create(flagFile)
	utils.Notifier("rgb(37FD12)", "Gamemode  [ON]")
}

func stopGm() {
	// reload hyprland to default settings
	utils.CmdRunner("hyprctl", "reload")
	err := os.Remove(flagFile)
	if err != nil {
		log.Default().Println("No gamemode file found. No changes made.")
	}
	utils.Notifier("rgb(E3242B)", "Gamemode [OFF]")
}

func toggleGM() {
	// always trigger start for first time run after login
	if !fs.FileExists(flagFile) {
		startGm()
	} else {
		stopGm()
	}
}
