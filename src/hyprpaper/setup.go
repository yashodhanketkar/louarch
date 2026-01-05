package hyprpaper

import (
	"os/exec"
)

func setupHyprpaper(selected string) {
	restartHyprpaperProcess()
	generateColorPallete(selected)
	setupTheme()
}

func restartHyprpaperProcess() {
	// reload hyprpaper process to apply changes
	_ = exec.Command("pkill", "hyprpaper").Run()
	cmdRunner("hyprctl", "dispatch", "exec", "hyprpaper")
}

func generateColorPallete(selected string) {
	// generate new color pallete from wallpaper of primary monitor
	cmdRunner("wallust", "run", "-q", "-u", WallpaperDir+"/"+selected)
}

func setupTheme() {
	// setup waybar theme
	// start new waybar instance with theme
	// restart waybar if it's running
	_ = exec.Command("pkill", "waybar").Run()
	cmdRunner("hyprctl", "-q", "dispatch", "exec", "waybar")

	// setup swaync theme
	// apply config and css changes
	ok := exec.Command("pidof", "swaync").Run()
	if ok != nil {
		cmdRunner("hyprctl", "-q", "dispatch", "exec", "swaync")
	}
	cmdRunner("swaync-client", "-rs")

	// source tmux colors if tmux is running to apply colors
	_ = exec.Command("tmux", "source-file", "~/.config/tmux/colors.conf").Run()

	// send color change signal to kitty if it's running
	kittyPid, err := exec.Command("pgrep", "kitty").Output()
	if err == nil {
		_ = exec.Command("kill", "-SIGUSR1", string(kittyPid)).Run()
	}
}
