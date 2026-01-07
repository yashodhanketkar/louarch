package hyprpaper

import (
	"fmt"
	"log"
	"os"
	"os/exec"

	"github.com/yashodhanketkar/louarch/src/utils"
)

func updateConfig(monitors []Monitor, wallpapers []string) {
	f, err := os.Create(utils.AppConfig.ConfigPath)
	if err != nil {
		log.Fatalf("failed to write config: %v", err)
	}
	defer f.Close()

	fmt.Fprintln(f, "ipc = true")
	fmt.Fprint(f, "splash = false\n\n")

	for i, m := range monitors {
		wp := wallpapers[i%len(wallpapers)]
		fmt.Fprintf(f,
			`wallpaper {
  monitor = %s
  path = %s/%s
}
`, m.Name, utils.AppConfig.WallpaperDir, wp)
	}
}

func setupHyprpaper(selected string) {
	restartHyprpaperProcess()
	generateColorPallete(selected)
	setupTheme()
}

func restartHyprpaperProcess() {
	// reload hyprpaper process to apply changes
	_ = exec.Command("pkill", "hyprpaper").Run()
	utils.CmdRunner("hyprctl", "dispatch", "exec", "hyprpaper")
}

func generateColorPallete(selected string) {
	// generate new color pallete from wallpaper of primary monitor
	utils.CmdRunner("wallust", "run", "-q", "-u", utils.AppConfig.WallpaperDir+"/"+selected)
}

func setupTheme() {
	// setup waybar theme
	// start new waybar instance with theme
	// restart waybar if it's running
	_ = exec.Command("pkill", "waybar").Run()
	utils.CmdRunner("hyprctl", "-q", "dispatch", "exec", "waybar")

	// setup swaync theme
	// apply config and css changes
	ok := exec.Command("pidof", "swaync").Run()
	if ok != nil {
		utils.CmdRunner("hyprctl", "-q", "dispatch", "exec", "swaync")
	}
	utils.CmdRunner("swaync-client", "-rs")

	// source tmux colors if tmux is running to apply colors
	_ = exec.Command("tmux", "source-file", "~/.config/tmux/colors.conf").Run()

	// send color change signal to kitty if it's running
	kittyPid, err := exec.Command("pgrep", "kitty").Output()
	if err == nil {
		_ = exec.Command("kill", "-SIGUSR1", string(kittyPid)).Run()
	}
}
