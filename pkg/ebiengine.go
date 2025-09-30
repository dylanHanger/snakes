package pkg

import (
	"context"
	"fmt"
	"sync"
	"time"

	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/inpututil"
	"github.com/jwalton/gchalk"
)

type ebiEngine[S, A any] struct {
	game     Game[S, A]
	renderer ebitenRenderable

	// lifetime management
	ctx    context.Context
	cancel context.CancelFunc

	// goroutine coordination
	mu          sync.Mutex
	turnLimiter chan struct{}

	// turn control
	turnDuration time.Duration
	isPaused     bool
	isStepping   bool
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
	turnDuration := CalculateTurnDuration(g.TurnsPerSecond())
	return &ebiEngine[S, A]{
		game:     g,
		renderer: r,

		ctx:    ctx,
		cancel: cancel,

		turnLimiter:  make(chan struct{}, 1),
		turnDuration: turnDuration,
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

	err := e.game.Reset()
	if err != nil {
		return err
	}

	ctx := context.Background()
	var wg sync.WaitGroup
	for id, p := range e.game.Players() {
		wg.Add(1)
		go func(p Player[S, A]) {
			err := p.Agent().Start(ctx)
			if err != nil {
				fmt.Printf("Error starting agent %d: %v", id, err)
			}
			wg.Done()
		}(p)
	}
	wg.Wait()
	e.Listen()

	e.markTurnReady()
	return ebiten.RunGame(e)
}

// launches goroutines for listening to each agent's talk channels
func (e *ebiEngine[S, A]) Listen() {
	for _, p := range e.game.Players() {
		if !p.Silent() {
			go e.listenTo(p, context.TODO())
		}
	}
}

func (e *ebiEngine[S, A]) listenTo(p Player[S, A], ctx context.Context) {
	a := p.Agent()
	c, ok := a.(Chatty)
	if !ok {
		fmt.Printf("%s is not chatty\n", p.Name())
		return
	}

	for msg := range c.Talk(ctx) {
		r, g, b, _ := p.Color().RGBA()
		colfn := gchalk.RGB(uint8(r>>8), uint8(g>>8), uint8(b>>8))
		fmt.Printf("[%s]: %s\n", colfn(p.Name()), msg)
	}
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
				if err := e.processTurn(); err != nil {
					e.cancel()
				}
			}()
			e.isPaused = e.isStepping
		default:
			// don't block interaction just because the turn isn't supposed to simulate
		}
	}

	// Should the game end?
	if e.game.IsGameOver() {
		// return ebiten.Termination
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

type agentReply[A any] struct {
	id     int
	err    error
	action A
	time   time.Duration
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

func (e *ebiEngine[S, A]) processTurn() error {
	turnCtx, turnCancel := context.WithCancel(e.ctx)
	defer turnCancel()

	replies := make(chan agentReply[A], len(e.game.Players()))

	// Communicate with each agent
	var wg sync.WaitGroup
	for id, p := range e.game.Players() {
		if e.game.ShouldSendState(id) {
			wg.Add(1)
			go func(id int, p Player[S, A]) {
				a := p.Agent()

				agentCtx, agentCancel := context.WithCancel(turnCtx)
				defer agentCancel()
				defer wg.Done()

				effectiveCtx := agentCtx
				if !p.WaitFor() {
					var cancel context.CancelFunc
					realTimeout := e.turnDuration
					if p.Timeout() > 0 {
						realTimeout = p.Timeout()
					}
					effectiveCtx, cancel = context.WithTimeout(agentCtx, realTimeout)
					defer cancel()
				}

				// Send the state out
				e.mu.Lock() // NOTE: Why lock here?
				reply, err := a.Send(e.game.State(id), effectiveCtx)
				e.mu.Unlock()

				startTime := time.Now()
				if err != nil {
					replies <- agentReply[A]{id: id, err: err}
					return
				}

				// Wait for the response (or timeout, if configured)
				// TODO: only accept the move if the channel is closed, otherwise accept the last submitted one?
				select {
				case r := <-reply:
					replies <- agentReply[A]{id: id, action: r, time: time.Since(startTime)}
				case <-effectiveCtx.Done():
					select {
					// Check again in case of a tiebreak situation
					case r := <-reply:
						replies <- agentReply[A]{id: id, action: r, time: time.Since(startTime)}
					default:
						replies <- agentReply[A]{id: id, err: effectiveCtx.Err(), time: time.Since(startTime)}
					}
				}
			}(id, p)
		}
	}

	// Ensure that the turn waits for at least the configured turn time
	// by creating a "phantom player" that holds up the wait group
	wg.Add(1)
	go func(ctx context.Context) {
		e.waitForTurn(ctx)
		wg.Done()
	}(turnCtx)

	// Ensure that the replies channel gets closed once all agents have replied
	go func() {
		wg.Wait()
		e.markTurnReady()
		close(replies)
	}()

	// Collect actions
	actions := make(map[int]A)
	for reply := range replies {
		actions[reply.id] = reply.action
	}

	e.mu.Lock()
	defer e.mu.Unlock()
	// Feed the actions collected from the agents to the game rules
	if err := e.game.ProcessTurn(actions); err != nil {
		return err
	}
	return nil
}

// waits until a timeout or the context is canceled
func (e *ebiEngine[S, A]) waitForTurn(ctx context.Context) {
	timeout, cancel := context.WithTimeout(ctx, e.turnDuration)

	select {
	case <-ctx.Done():
	case <-timeout.Done():
	}
	cancel()
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
	arenaWidth := screenWidth //int(float64(screenWidth) * 0.8)
	arenaHeight := screenHeight

	// Create arena image with calculated dimensions
	arenaScreen := ebiten.NewImage(arenaWidth, arenaHeight)

	e.mu.Lock()
	e.renderer.Render(arenaScreen)
	e.mu.Unlock()

	// Draw arena to the left side of the screen (starting at x=0)
	op := &ebiten.DrawImageOptions{}
	screen.DrawImage(arenaScreen, op)
}

// Layout implements ebiten.Game
func (e *ebiEngine[S, A]) Layout(outsideWidth, outsideHeight int) (insideWidth, insideHeight int) {
	return outsideWidth, outsideHeight
}
