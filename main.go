package main

import (
	"github.com/yashodhanketkar/louarch/src/cmd"
	"github.com/yashodhanketkar/louarch/src/utils"
)

func main() {
	utils.Configure()
	cmd.Execute()
}
