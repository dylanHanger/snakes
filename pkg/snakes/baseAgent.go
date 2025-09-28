package snakes

import (
	"context"
	"fmt"
)

type baseAgent struct {
	ctx  context.Context
	talk chan string
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
