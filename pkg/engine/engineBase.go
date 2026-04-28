package engine

import (
	"context"
	"fmt"
	"sync"
	"time"

	"github.com/jwalton/gchalk"
)

type agentReply[A any] struct {
	id     int
	err    error
	action A
	time   time.Duration
}

// engineBase contains the shared turn-execution logic used by any engines.
// It fans out the game state to every player's agent, collects moves 
// under per-player WaitFor/Timeout rules, enforces a minimum turn duration, 
// and applies the resulting actions to the game.
type engineBase[S, A any] struct {
	game Game[S, A]

	// mu serializes ProcessTurn against renderers that read game state.
	// Concurrent readers (e.g. an ebiten Draw) should take it via Lock/Unlock.
	mu sync.Mutex

	turnDuration time.Duration
}

func newEngineBase[S, A any](g Game[S, A]) *engineBase[S, A] {
	return &engineBase[S, A]{
		game:         g,
		turnDuration: CalculateTurnDuration(g.TurnsPerSecond()),
	}
}

func (r *engineBase[S, A]) Lock()   { r.mu.Lock() }
func (r *engineBase[S, A]) Unlock() { r.mu.Unlock() }

// StartAgents starts every player's agent and blocks until all Start calls return.
func (r *engineBase[S, A]) StartAgents(ctx context.Context) {
	var wg sync.WaitGroup
	for id, p := range r.game.Players() {
		wg.Add(1)
		go func(id int, p Player[S, A]) {
			defer wg.Done()
			if err := p.Agent().Start(ctx); err != nil {
				fmt.Printf("Error starting agent %d: %v\n", id, err)
			}
		}(id, p)
	}
	wg.Wait()
}

// Listen launches a goroutine per non-silent Chatty player that prints
// messages to stdout colored by player color.
func (r *engineBase[S, A]) Listen(ctx context.Context) {
	for _, p := range r.game.Players() {
		go r.listenTo(p, ctx)
	}
}

func (r *engineBase[S, A]) listenTo(p Player[S, A], ctx context.Context) {
	c, ok := p.Agent().(Chatty)
	if !ok {
		return
	}
	for msg := range c.Talk(ctx) {
		red, green, blue, _ := p.Color().RGBA()
		colfn := gchalk.RGB(uint8(red>>8), uint8(green>>8), uint8(blue>>8))
		if (!p.Silent()) {
			fmt.Printf("[%s]: %s\n", colfn(p.Name()), msg)
		}
	}
}

// ProcessTurn runs one turn: fan out state to agents, collect their replies
// (respecting WaitFor/Timeout), wait at least turnDuration, then apply the
// collected actions to the game.
func (r *engineBase[S, A]) ProcessTurn(parent context.Context) error {
	turnCtx, turnCancel := context.WithCancel(parent)
	defer turnCancel()

	replies := make(chan agentReply[A], len(r.game.Players()))

	var wg sync.WaitGroup
	for id, p := range r.game.Players() {
		s, err := r.game.State(id)
		if err != nil {
			continue
		}
		wg.Add(1)
		go func(id int, p Player[S, A]) {
			defer wg.Done()
			a := p.Agent()

			var ctx context.Context
			var cancel context.CancelFunc
			if p.WaitFor() {
				ctx, cancel = context.WithCancel(turnCtx)
			} else {
				timeout := r.turnDuration
				if p.Timeout() > 0 {
					timeout = p.Timeout()
				}
				ctx, cancel = context.WithTimeout(turnCtx, timeout)
			}
			defer cancel()

			startTime := time.Now()
			reply, err := a.Send(s, ctx)
			if err != nil {
				replies <- agentReply[A]{id: id, err: err}
				return
			}

			select {
			case rv := <-reply:
				replies <- agentReply[A]{id: id, action: rv, time: time.Since(startTime)}
			case <-ctx.Done():
				t := time.Since(startTime)
				select {
				// Check again in case of a tiebreak situation
				case rv := <-reply:
					replies <- agentReply[A]{id: id, action: rv, time: t}
				default:
					replies <- agentReply[A]{id: id, err: ctx.Err(), time: t}
				}
			}
		}(id, p)
	}

	// phantom player that holds the wait group open until the minimum turn
	// duration has elapsed
	wg.Add(1)
	go func(ctx context.Context) {
		defer wg.Done()
		timeout, cancel := context.WithTimeout(ctx, r.turnDuration)
		defer cancel()
		select {
		case <-ctx.Done():
		case <-timeout.Done():
		}
	}(turnCtx)

	go func() {
		wg.Wait()
		close(replies)
	}()

	actions := make(map[int]A)
	for reply := range replies {
		actions[reply.id] = reply.action
	}

	r.mu.Lock()
	defer r.mu.Unlock()
	return r.game.ProcessTurn(actions)
}
