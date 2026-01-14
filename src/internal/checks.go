package internal

import (
	"log"
	"os/exec"
)

const (
	Wofi int = iota
	Hyprpaper
)

func EarlyExit(process int) {
	switch process {
	case Wofi:
		checkWofi()
	case Hyprpaper:
		checkHyprpaper()
	}
}

func checkWofi() bool {

	if err := exec.Command("pidof", "wofi").Run(); err == nil {
		exec.Command("killall", "wofi").Run()
		log.Fatal("Wofi process is running, killing it...")
		return false
	}

	return false
}

func checkHyprpaper() int { return 1 }
