package fs

import (
	"os"

	"github.com/yashodhanketkar/louarch/src/utils"
)

type Config = utils.Config

var (
	configFile = os.ExpandEnv("$HOME/.config/louarch/config.json")
	AppConfig  = Config{}
)
