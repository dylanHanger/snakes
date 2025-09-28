package pkg

import (
	"context"
	"hash/fnv"
	"image/color"
	"math/rand/v2"
	"time"

	"github.com/hajimehoshi/ebiten/v2"
)

type Engine[S, A any] interface {
	Run() error
}

type Game[S, A any] interface {
	Name() string
	TurnsPerSecond() float64

	Players() map[int]Player[S, A]

	ShouldSendState(id int) bool
	State(id int) S

	Reset() error
	ProcessTurn(actions map[int]A) error

	IsGameOver() bool
}

func GetRandomFromSeed(seed string) *rand.Rand {
	seedHash := fnv.New64()
	seedHash.Write([]byte(seed))
	seedValue := seedHash.Sum64()
	return rand.New(rand.NewPCG(seedValue, seedValue))
}

func CalculateTurnDuration(turnsPerSecond float64) time.Duration {
	if turnsPerSecond <= 0 {
		return time.Duration(0)
	}
	secondsPerTurn := 1.0 / turnsPerSecond
	return time.Duration(secondsPerTurn * float64(time.Second))
}

type ebitenRenderable interface {
	Render(screen *ebiten.Image)
}

type PlayerConfig interface {
	// the name of the player
	Name() string
	// the color of the player
	Color() color.Color

	// should this agent be silenced whent talking?
	Silent() bool
	// should the engine always wait for this player to make a move?
	WaitFor() bool
	// how long should the engine give this player to make a move?
	Timeout() time.Duration
}
type Player[S, A any] interface {
	PlayerConfig
	// should always return the same instance of the agent
	Agent() Agent[S, A]
}

type Agent[S any, A any] interface {
	// start the agent
	//
	// ctx represents the lifetime of the agent, it should shut down cleanly when the context cancels
	Start(ctx context.Context) error

	// Get an action from the agent
	//
	// Send may block while sending the state, and then output the move on the channel after any time
	Send(state S, ctx context.Context) (<-chan A, error)
}

type Chatty interface {
	// Get a channel that outputs chat messages
	Talk(ctx context.Context) <-chan string
}
