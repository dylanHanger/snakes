package snakes

import (
	"context"

	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/inpututil"
)

type keyboardAgent struct {
	baseAgent
	bindings    []keyBinding
	inputBuffer chan Direction
}

type keyBinding struct {
	direction Direction
	key       ebiten.Key
}

func NewKeyboardAgent(keyMap map[Direction]ebiten.Key) *keyboardAgent {
	bindings := make([]keyBinding, 0, len(keyMap))
	for direction, key := range keyMap {
		bindings = append(bindings, keyBinding{direction: direction, key: key})
	}
	return &keyboardAgent{
		bindings:    bindings,
		inputBuffer: make(chan Direction),
	}
}

func (a *keyboardAgent) PollInput() {
	if a.ctx != nil {
		select {
		case <-a.ctx.Done():
			return
		default:
		}
	}

	for _, binding := range a.bindings {
		if inpututil.IsKeyJustPressed(binding.key) {
			select {
			case a.inputBuffer <- binding.direction:
			default:
			}
		}
	}
}

// Send implements pkg.Agent.
func (a *keyboardAgent) Send(state State, ctx context.Context) (<-chan Direction, error) {
	c := make(chan Direction)
	go func() {
		defer close(c)

		select {
		case d := <-a.inputBuffer:
			c <- d
		case <-ctx.Done():
		}
	}()
	return c, nil
}
