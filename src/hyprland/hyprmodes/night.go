package hyprmodes

import (
	"os/exec"
	"time"

	"github.com/yashodhanketkar/louarch/src/utils"
)

func startNm() {
	utils.CmdRunner("hyprctl", "dispatch", "exec", "hyprsunset")
	time.Sleep(time.Millisecond * 200) // avoids race condition
	utils.CmdRunner("hyprctl", "hyprsunset", "temperature", "4500")
}

func toggleNM() {
	if err := exec.Command("pidof", "hyprsunset").Run(); err != nil {
		startNm()
	} else {
		utils.CmdRunner("killall", "hyprsunset")
	}
}
