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
	"github.com/jwalton/gchalk"
)

//go:embed icon.png
var Icon []byte

// TODO: Scoring
// TODO: UI (display winner, scoreboard, game over, restart, pause, resume, step buttons)
func main() {
	// Define command line flags
	configFlag := flag.String("config", "config.yaml", "Path to config file")
	headlessFlag := flag.Bool("headless", false, "Run in headless mode")
	flag.Parse()

	configPath := *configFlag
	cfg, err := snakes.LoadConfig(configPath)
	if err != nil {
		fmt.Printf("Could not load config `%s`: %v\n", configPath, err)
		cfg = snakes.DefaultConfig()
	}

	g := snakes.NewGame(cfg)
	var e engine.Engine[snakes.State, snakes.Direction]
	if *headlessFlag {
		e = engine.NewHeadlessEngine(g)
	} else {
		e = engine.NewEbitenEngine(g)
		img, _, _ := image.Decode(bytes.NewReader(Icon))
		ebiten.SetWindowIcon([]image.Image{img})
	}
	e.Run()
	scoreboard := g.Scoreboard()
	for i,s := range scoreboard {
		r, g, b, _ := s.Player.Color().RGBA()
		colfn := gchalk.RGB(uint8(r>>8), uint8(g>>8), uint8(b>>8))
		fmt.Printf("%d. %s: %v\n", i+1, colfn(s.Player.Name()), s.Score)
	}
}
