package utils

import (
	"fmt"
	"log"
	"os"
)

func CreateFile(path string, args ...string) {
	for _, arg := range args {
		fmt.Println(arg)
	}

	file, err := os.Create(path)
	if err != nil {
		log.Fatalf("Error creating file %s\n%v", path, err)
	}
	defer file.Close()
}
