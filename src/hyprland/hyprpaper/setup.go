package hyprpaper

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"os/exec"

	"github.com/yashodhanketkar/louarch/src/fs"
	"github.com/yashodhanketkar/louarch/src/utils"
)

func updateConfig(monitors []Monitor, wallpapers []string, wpDir string) {
	file, err := os.Create(fs.AppConfig.ConfigPath)
	if err != nil {
		log.Fatalf("failed to write config: %v", err)
	}
	defer file.Close()
	w := bufio.NewWriter(file)
	defer w.Flush()

	w.WriteString("ipc = true\n")
	w.WriteString("splash = false\n\n")

	for i, m := range monitors {
		wp := wallpapers[i%len(wallpapers)]
		mName := ""
		if i != 0 {
			mName = m.Name
		}
		fmt.Fprintf(w, `wallpaper {
  monitor = %s
  path = %s/%s
}
`, mName, wpDir, wp)
	}
}

func setupHyprpaper(selected string, wpDir string) {
	restartHyprpaperProcess()
	generateColorPallete(selected, wpDir)
	setupTheme()
}

func restartHyprpaperProcess() {
	// reload hyprpaper process to apply changes
	_ = exec.Command("pkill", "hyprpaper").Run()
	utils.CmdRunner("hyprctl", "dispatch", "exec", "hyprpaper")
}

func generateColorPallete(selected, wpDir string) {
	// generate new color pallete from wallpaper of primary monitor
	utils.CmdRunner("wallust", "run", "-q", "-u", wpDir+"/"+selected)
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
