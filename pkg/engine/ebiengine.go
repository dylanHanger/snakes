package engine

import (
	"context"
	"fmt"
	"strings"

	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/ebitenutil"
	"github.com/hajimehoshi/ebiten/v2/inpututil"
	"github.com/jwalton/gchalk"
)

type ebiEngine[S, A any] struct {
	*engineBase[S, A]

	renderer ebitenRenderable

	ctx    context.Context
	cancel context.CancelFunc

	turnLimiter chan struct{}

	isPaused   bool
	isStepping bool
}

type inputPoller interface {
	PollInput()
}

func NewEbitenEngine[S, A any](g Game[S, A]) Engine[S, A] {
	r, ok := g.(ebitenRenderable)
	if !ok {
		panic("game is not renderable by ebiten")
	}

	ctx, cancel := context.WithCancel(context.Background())
	return &ebiEngine[S, A]{
		engineBase:  newEngineBase[S, A](g),
		renderer:    r,
		ctx:         ctx,
		cancel:      cancel,
		turnLimiter: make(chan struct{}, 1),
	}
}

// Run implements Engine
// TODO: count turns per second (how many times a second does processTurn get called)
func (e *ebiEngine[S, A]) Run() error {
	ebiten.SetWindowSize(800, 800)
	ebiten.SetWindowResizingMode(ebiten.WindowResizingModeEnabled)
	ebiten.SetRunnableOnUnfocused(true)
	ebiten.SetWindowTitle(e.game.Name())

	ebiten.SetVsyncEnabled(true)
	ebiten.SetTPS(ebiten.SyncWithFPS)

	if err := e.game.Reset(); err != nil {
		return err
	}

	e.StartAgents(context.Background())
	e.Listen(context.TODO())

	e.markTurnReady()
	return ebiten.RunGame(e)
}

// Update implements ebiten.Game
func (e *ebiEngine[S, A]) Update() error {
	if inpututil.IsKeyJustPressed(ebiten.KeySpace) {
		e.togglePause()
	}
	if inpututil.IsKeyJustPressed(ebiten.KeyEnter) {
		e.singleStep()
	}

	e.pollAgentInputs()

	shouldProcessTurn := !e.isPaused && !e.game.IsGameOver()
	if shouldProcessTurn {
		select {
		case <-e.turnLimiter:
			go func() {
				if err := e.ProcessTurn(e.ctx); err != nil {
					e.cancel()
				}
				e.markTurnReady()
			}()
			e.isPaused = e.isStepping
		default:
			// don't block interaction just because the turn isn't supposed to simulate
		}
	}

	// Should the game end?
	if e.game.IsGameOver() {
		scoreboard := e.game.Scoreboard()
		for i,s := range scoreboard {
			r, g, b, _ := s.Player.Color().RGBA()
			colfn := gchalk.RGB(uint8(r>>8), uint8(g>>8), uint8(b>>8))
			fmt.Printf("%d. %s: %v\n", i+1, colfn(s.Player.Name()), s.Score)
		}
		return ebiten.Termination
	}
	return nil
}

func (e *ebiEngine[S, A]) pollAgentInputs() {
	for _, p := range e.game.Players() {
		if poller, ok := p.Agent().(inputPoller); ok {
			poller.PollInput()
		}
	}
}

func (e *ebiEngine[S, A]) singleStep() {
	e.isStepping = true
	e.isPaused = false
}

func (e *ebiEngine[S, A]) togglePause() {
	e.isPaused = !e.isPaused
	if !e.isPaused {
		e.isStepping = false
	}
}

func (e *ebiEngine[S, A]) markTurnReady() {
	e.turnLimiter <- struct{}{}
}

// Draw implements ebiten.Game
func (e *ebiEngine[S, A]) Draw(screen *ebiten.Image) {
	// TODO: allow the game (or a renderer) to define a UI somehow, with buttons etc

	// Get full screen dimensions
	screenWidth, screenHeight := screen.Bounds().Dx(), screen.Bounds().Dy()

	// Calculate arena dimensions (left 80% of screen, full height)
	arenaWidth := int(float64(screenWidth) * 0.8)
	arenaHeight := screenHeight
	scoreWidth := screenWidth - arenaWidth

	// Create arena image with calculated dimensions
	arenaScreen := ebiten.NewImage(arenaWidth, arenaHeight)
	scoreScreen := ebiten.NewImage(scoreWidth, arenaHeight)

	e.Lock()
	e.renderer.Render(arenaScreen)
	e.drawScoreboard(scoreScreen)
	e.Unlock()

	// Draw arena to the left side of the screen (starting at x=0)
	op := &ebiten.DrawImageOptions{}
	screen.DrawImage(arenaScreen, op)

	op = &ebiten.DrawImageOptions{}
	op.GeoM.Translate(float64(arenaWidth), 0)
	screen.DrawImage(scoreScreen, op)
}

func (e *ebiEngine[S, A]) drawScoreboard(screen *ebiten.Image) {
	entries := e.game.Scoreboard()
	if len(entries) == 0 {
		return
	}
	lines := make([]string, len(entries))
	for i, entry := range entries {
		lines[i] = fmt.Sprintf("%d. %s: %s", i+1, entry.Player.Name(), entry.Score)
	}
	ebitenutil.DebugPrint(screen, strings.Join(lines, "\n"))
}

// Layout implements ebiten.Game
func (e *ebiEngine[S, A]) Layout(outsideWidth, outsideHeight int) (insideWidth, insideHeight int) {
	return outsideWidth, outsideHeight
}
