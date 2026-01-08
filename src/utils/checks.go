package utils

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
		CheckWofi()
	case Hyprpaper:
		CheckHyprpaper()
	}
}

func CheckWofi() bool {

	if err := exec.Command("pidof", "wofi").Run(); err == nil {
		exec.Command("killall", "wofi").Run()
		log.Fatal("Wofi process is running, killing it...")
		return false
	}

	return false
}

func CheckHyprpaper() int { return 1 }
