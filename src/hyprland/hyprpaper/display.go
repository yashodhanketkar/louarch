package hyprpaper

import (
	"encoding/json"
	"fmt"
	"log"
	"os"
	"path/filepath"
	"strings"

	"github.com/yashodhanketkar/louarch/src/fs"
	"github.com/yashodhanketkar/louarch/src/utils"
)

func listMonitors() []Monitor {
	out := utils.CmdRunner("hyprctl", "monitors", "-j")

	var mons []Monitor
	if err := json.Unmarshal([]byte(out), &mons); err != nil {
		log.Fatalf("failed to parse monitors json: %v", err)
	}

	return mons
}

func listWallpapers() []string {
	entries, err := os.ReadDir(fs.AppConfig.WallpaperDir)
	if err != nil {
		log.Fatalf("failed to read wallpaper dir: %v", err)
	}

	var files []string
	for _, e := range entries {
		if e.IsDir() {
			continue
		}
		ext := strings.ToLower(filepath.Ext(e.Name()))
		switch ext {
		case ".jpg", ".jpeg", ".png", ".webp":
			files = append(files, e.Name())
		}
	}

	if len(files) == 0 {
		log.Fatal("no wallpapers found")
	}
	return files
}

func selectWallpapers(wallpapers []string, monitors []Monitor) ([]string, bool) {
	selected := make([]string, 0)
	nos := len(monitors)
	i := 0

	for i <= nos {
		wp, ok := utils.WofiPrompt(
			fmt.Sprintf("Select Wallpaper for %v", monitors[i].Name),
			wallpapers...,
		)
		if !ok {
			utils.WofiPrompt("Exiting without selecting...", "Ok")
			return nil, false
		}

		selected = append(selected, wp)
		i = i + 1
		// early exit if all wallpapers slots are filled
		if len(selected) == nos {
			break
		}

		continueChoice, _ := utils.WofiPrompt("Select more", "No", "Yes")
		switch continueChoice {
		case "Yes":
			continue
		case "No":
			return selected, true
		default:
			selected = selected[:len(selected)-1]
			i = i - 1
		}
	}

	return selected, true
}

func wallSwitcher() {
	monitors := listMonitors()
	wallpapers := listWallpapers()

	selected, ok := selectWallpapers(wallpapers, monitors)
	if !ok {
		return
	}

	wpDir := fs.AppConfig.WallpaperDir
	updateConfig(monitors, selected, wpDir)
	setupHyprpaper(selected[0], wpDir)
}

func setWallpaper(path string) {
	wpDir, wallpaper := filepath.Split(path)
	updateConfig(listMonitors(), []string{wallpaper}, wpDir)
	setupHyprpaper(wallpaper, wpDir)
}
