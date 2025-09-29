package snakes

import (
	"context"
	"fmt"

	"github.com/dylanHanger/snakes/pkg"
)

type baseAgent struct {
	ctx    context.Context
	talk   chan string
	random *pkg.SharedRand
}

func (a *baseAgent) Start(ctx context.Context) error {
	a.ctx = ctx
	return nil
}
func (a *baseAgent) Stop(ctx context.Context) error { return nil }

func (a *baseAgent) TrySay(msg string, args ...any) bool {
	if a.talk == nil {
		return false
	}
	a.talk <- fmt.Sprintf(msg, args...)
	return true
}

func (a *baseAgent) Talk(ctx context.Context) <-chan string {
	if a.talk == nil {
		a.talk = make(chan string)
		go func() {
			<-ctx.Done()
			close(a.talk)
		}()
	}
	return a.talk
}

func (a *baseAgent) SetRandom(r *pkg.SharedRand) {
	a.random = r
}

func (a *baseAgent) Random() *pkg.SharedRand {
	return a.random
}
