package snakes

import "context"

type randomAgent struct {
	baseAgent
}

func NewRandomAgent() *randomAgent {
	return &randomAgent{}
}

func (a *randomAgent) Start(ctx context.Context) error {
	return nil
}

func (a *randomAgent) Send(state State, ctx context.Context) (<-chan Direction, error) {
	c := make(chan Direction, 1)
	r := a.random
	if r != nil {
		c <- DirCardinals[r.IntN(len(DirCardinals))]
	} else {
		c <- DirCardinals[0]
	}
	close(c)
	return c, nil
}
