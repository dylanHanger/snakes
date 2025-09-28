package snakes

import (
	"context"

	"github.com/hajimehoshi/ebiten/v2"
)

type keyboardAgent struct {
	baseAgent
	keyMap map[Direction]ebiten.Key
}

func NewKeyboardAgent(keyMap map[Direction]ebiten.Key) *keyboardAgent {
	return &keyboardAgent{
		keyMap: keyMap,
	}
}

// Send implements pkg.Agent.
func (a *keyboardAgent) Send(state State, ctx context.Context) (<-chan Direction, error) {
	c := make(chan Direction)

	myConfig := state.Players[state.Id]

	go func() {
		timeout, cancel := context.WithTimeout(ctx, myConfig.Timeout())
		defer cancel()
		defer close(c)

		var d Direction

		for {
			// Check for key presses
			for direction, key := range a.keyMap {
				if ebiten.IsKeyPressed(key) {
					d = direction
					if myConfig.WaitFor() && timeout.Err() == context.DeadlineExceeded {
						c <- d
						return
					}
				}
			}

			// Also check if the context is done
			select {
			case <-ctx.Done():
				c <- d
				return
			default:
				// Keep polling for key presses
			}
		}
	}()

	return c, nil
}
