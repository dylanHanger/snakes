package engine

import "context"

type headlessEngine[S, A any] struct {
	*engineBase[S, A]

	ctx    context.Context
	cancel context.CancelFunc
}

func NewHeadlessEngine[S, A any](g Game[S, A]) Engine[S, A] {
	ctx, cancel := context.WithCancel(context.Background())
	return &headlessEngine[S, A]{
		engineBase: newEngineBase(g),
		ctx:        ctx,
		cancel:     cancel,
	}
}

// Run implements [Engine].
func (e *headlessEngine[S, A]) Run() error {
	if err := e.game.Reset(); err != nil {
		return err
	}

	e.StartAgents(context.Background())
	e.Listen(e.ctx)

	for !e.game.IsGameOver() {
		select {
		case <-e.ctx.Done():
			return e.ctx.Err()
		default:
		}
		if err := e.ProcessTurn(e.ctx); err != nil {
			e.cancel()
			return err
		}
	}

	return nil
}

