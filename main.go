package main

import (
	"github.com/yashodhanketkar/louarch/src/cmd"
	"github.com/yashodhanketkar/louarch/src/fs"
)

func main() {
	fs.Configure()
	cmd.Execute()
}
