package utils

import (
	"bytes"
	"log"
	"os/exec"
	"strings"
)

func CmdRunner(cmd string, args ...string) string {
	c := exec.Command(cmd, args...)
	out, err := c.CombinedOutput()
	if err != nil {
		log.Fatalf("command failed: %s %v\n%s", cmd, args, string(out))
	}
	return string(out)
}

func WofiPrompt(prompt string, options ...string) (string, bool) {
	cmd := exec.Command("wofi", "-W", "800", "-d", "-p", prompt)
	cmd.Stdin = strings.NewReader(strings.Join(options, "\n"))

	var out bytes.Buffer
	cmd.Stdout = &out

	err := cmd.Run()
	if err != nil {
		return "", false
	}

	choice := strings.TrimSpace(out.String())
	if choice == "" {
		return "", false
	}

	return choice, true
}
