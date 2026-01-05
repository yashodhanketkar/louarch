package hyprpaper

import (
	"bytes"
	"fmt"
	"log"
	"os"
	"os/exec"
	"strings"
)

func updateConfig(monitors []Monitor, wallpapers []string) {
	f, err := os.Create(ConfigPath)
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
`, m.Name, WallpaperDir, wp)
	}
}

func cmdRunner(cmd string, args ...string) string {
	c := exec.Command(cmd, args...)
	out, err := c.CombinedOutput()
	if err != nil {
		log.Fatalf("command failed: %s %v\n%s", cmd, args, string(out))
	}
	return string(out)
}

func wofiPrompt(prompt string, options ...string) (string, bool) {
	cmd := exec.Command("wofi", "-d", "-p", prompt)
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
