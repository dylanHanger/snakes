package snakes

import (
	"context"
	"fmt"
	"math/rand/v2"

	"github.com/dylanHanger/snakes/pkg"
)

type randomAgent struct {
	baseAgent

	random *rand.Rand
}

func NewRandomAgent() *randomAgent {
	return &randomAgent{}
}

func (a *randomAgent) Start(ctx context.Context) error {
	a.random = nil
	return nil
}

func (a *randomAgent) Send(state State, ctx context.Context) (<-chan Direction, error) {
	if a.random == nil {
		a.random = pkg.GetRandomFromSeed(fmt.Sprintf("%s%d", state.Seed, state.Id))
	}

	c := make(chan Direction, 1)
	c <- DirCardinals[a.random.IntN(4)]
	close(c)
	return c, nil
}
