package main

import (
	"bytes"
	"flag"
	"fmt"
	"image"

	_ "embed"

	"github.com/dylanHanger/snakes/pkg/engine"
	"github.com/dylanHanger/snakes/pkg/snakes"
	"github.com/hajimehoshi/ebiten/v2"
)

//go:embed icon.png
var Icon []byte

// TODO: Scoring
// TODO: UI (display winner, scoreboard, game over, restart, pause, resume, step buttons)
func main() {
	// Define command line flags
	configFlag := flag.String("config", "config.yaml", "Path to config file")
	flag.Parse()

	configPath := *configFlag
	cfg, err := snakes.LoadConfig(configPath)
	if err != nil {
		fmt.Printf("Could not load config `%s`: %v\n", configPath, err)
		cfg = snakes.DefaultConfig()
	}
	g := snakes.NewGame(cfg)
	e := engine.NewEbitenEngine(g)
	img, _, _ := image.Decode(bytes.NewReader(Icon))
	ebiten.SetWindowIcon([]image.Image{img})
	end := e.Run()
	if end == ebiten.Termination {
		println("Game over!")
	} else if end != nil {
		fmt.Printf("Something went wrong: %v", end)
	}
}
