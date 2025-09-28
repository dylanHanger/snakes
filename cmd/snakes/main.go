package main

import (
	"flag"
	"fmt"

	"github.com/dylanHanger/snakes/pkg"
	"github.com/dylanHanger/snakes/pkg/snakes"
)

// TODO: Fix Go's stupid fucking random behaviour that makes everything fucking non-deterministic
// TODO: Scoring
// TODO: UI (display winner, scoreboard, game over, restart, pause, resume, step buttons)
func main() {
	// Define command line flags
	configFlag := flag.String("config", "config/config.yaml", "Path to config file")
	flag.Parse()

	configPath := *configFlag
	cfg, err := snakes.LoadConfig(configPath)
	if err != nil {
		fmt.Printf("Could not load config %s: %v\n", configPath, err)
		cfg = snakes.DefaultConfig()
	}
	g := snakes.NewGame(cfg)
	e := pkg.NewEbitenEngine(g)
	e.Run()
}
