package utils

import (
	"encoding/json"
	"errors"
	"log"
	"os"
)

func CreateFile(path string, args ...string) {
	for _, arg := range args {
		log.Default().Print(arg)
	}

	file, err := os.Create(path)
	if err != nil {
		log.Fatalf("Error creating file %s\n%v", path, err)
	}
	defer file.Close()
}

func readConfig(configPath string) {
	data, err := os.ReadFile(configPath)
	if err != nil {
		log.Fatalf("failed to open configuration file. %v", err)
	}

	err = json.Unmarshal(data, &AppConfig)
	if err != nil {
		log.Fatalf("Failed to load configuration. %v", err)
	}
}

func FileExists(file string) bool {
	_, err := os.Stat(file)
	if errors.Is(err, os.ErrNotExist) {
		return false
	}
	return true
}

// func fileWriter(filePath string, data ...string) {
// 	file, err = os.st
// }
