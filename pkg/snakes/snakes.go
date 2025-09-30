package snakes

import (
	"errors"
	"fmt"
	"image/color"
	"math"
	"time"

	"github.com/dylanHanger/snakes/pkg"
	"github.com/jinzhu/copier"
)

// Fun Modifiers
// Power Apples: Invisibility, Invincibility, Intangibility, etc
// Zombie Snakes: NPC snakes that cause havoc
// Portals: Teleport around by moving through portals
// Battle Royal: The arena slowly shrinks
// Obstacles: The arena is filled with walls

var ErrNoSpace = errors.New("no space on the map")

type player = pkg.Player[State, Direction]

type (
	Game struct {
		config  *Config
		players map[int]player

		random      *pkg.SharedRand
		state       state
		renderState renderState
	}

	// internal state used by the game to simulate turns
	state struct {
		startTime   time.Time
		currentTurn int

		snakes map[int]*Snake
		food   map[GridPoint]int
	}
)

func NewGame(cfg *Config) *Game {
	g := &Game{
		config:  cfg,
		players: make(map[int]player),
	}
	for id, p := range cfg.Players {
		g.players[id] = p
	}
	return g
}

func NewPlayer(cfg PlayerConfig, a pkg.Agent[State, Direction]) *Player {
	return &Player{
		PlayerConfig: cfg,
		agent:        a,
	}
}

// Agent implements pkg.Player.
func (p *Player) Agent() pkg.Agent[State, Direction] { return p.agent }

// Color implements pkg.Player.
func (p *Player) Color() color.Color { return p.PlayerConfig.Color }

// Name implements pkg.Player.
func (p *Player) Name() string { return p.PlayerConfig.Name }

// Silent implements pkg.Player
func (p *Player) Silent() bool { return p.PlayerConfig.Silent }

// Timeout implements pkg.Player.
func (p *Player) Timeout() time.Duration { return p.PlayerConfig.Timeout }

// WaitFor implements pkg.Player.
func (p *Player) WaitFor() bool { return p.PlayerConfig.WaitFor }

// Name implements pkg.Game.
func (g *Game) Name() string { return "Snakes!" }

// TurnsPerSecond implements pkg.Game.
func (g *Game) TurnsPerSecond() float64 { return g.config.TurnsPerSecond }

// Players implements pkg.Game.
func (g *Game) Players() map[int]pkg.Player[State, Direction] {
	return g.players
}

// Reset implements pkg.Game.
func (g *Game) Reset() error {
	g.random = pkg.GetRandomFromSeed(g.config.Seed)
	g.state.startTime = time.Now()
	g.state.currentTurn = 0
	g.state.snakes = make(map[int]*Snake)
	g.state.food = make(map[GridPoint]int)

	for id, player := range g.players {
		g.state.snakes[id] = new(Snake)
		if setter, ok := player.Agent().(interface{ SetRandom(*pkg.SharedRand) }); ok {
			setter.SetRandom(pkg.GetRandomFromSeed(fmt.Sprintf("%s:%d", g.config.Seed, id)))
		}
	}

	g.spawnSnakes()
	g.spawnFood()

	return nil
}

func (g *Game) ShouldSendState(id int) bool {
	snake, exists := g.state.snakes[id]
	return exists && snake != nil && !snake.IsDead()
}

// State implements pkg.Game.
func (g *Game) State(id int) (State, error) {
	food := make(map[GridPoint]int)
	copier.Copy(&food, g.state.food)

	snakes := make(map[int]Snake)
	copier.Copy(&snakes, g.state.snakes)

	if g.ShouldSendState(id) {
		return State{
			Id:     id,
			Config: g.config,
			Snakes: snakes,
			Food:   food,
		}, nil
	}
	return State{}, fmt.Errorf("player should not receive state this turn")
}

// ProcessTurn implements pkg.Game.
func (g *Game) ProcessTurn(actions map[int]Direction) error {
	g.state.currentTurn++
	for id, move := range actions {
		snake := g.state.snakes[id]
		if snake.IsDead() {
			// just in case we asked for a move from a dead snake
			continue
		}
		snake.Move(move)
	}

	g.processCollisions()
	// rot food
	if g.config.FoodLifetime > 0 {
		for p, v := range g.state.food {
			if v <= 1 {
				delete(g.state.food, p)
				continue
			}
			g.state.food[p] = v - 1
		}
	}

	g.spawnSnakes()
	g.spawnFood()

	return nil
}

func (g *Game) findEmptyPoint() (GridPoint, error) {
	occupied := make(map[GridPoint]bool)
	for p := range g.state.food {
		occupied[p] = true
	}
	for _, s := range g.state.snakes {
		for _, b := range s.body {
			occupied[b] = true
		}
	}

	emptyPoints := []GridPoint{}
	for y := range g.config.Height {
		for x := range g.config.Width {
			p := GridPoint{X: x, Y: y}
			if !occupied[p] {
				emptyPoints = append(emptyPoints, p)
			}
		}
	}

	if len(emptyPoints) == 0 {
		return GridPoint{}, ErrNoSpace
	}

	return emptyPoints[g.random.IntN(len(emptyPoints))], nil
}

func (g *Game) spawnFood() {
	requiredFood := g.config.FoodCount - len(g.state.food)
	for range requiredFood {
		p, err := g.findEmptyPoint()
		if err == nil {
			g.state.food[p] = g.config.FoodLifetime
		}
	}
}

func (g *Game) spawnSnakes() {
	for i := range len(g.state.snakes) {
		// indexed like this to guarantee iteration order
		s := g.state.snakes[i]
		if s.IsDead() {
			s.respawnCounter--
			if s.respawnCounter < 0 {
				g.spawnSnake(s)
			}
		}

	}
}

func (g *Game) spawnSnake(s *Snake) {
	p, err := g.findEmptyPoint()
	if err == nil {
		s.body = []GridPoint{p}
		s.length = 5
		s.respawnCounter = g.config.RespawnTime
	}
}

func (g *Game) getFoodValue(lifetime int) int {
	if g.config.FoodLifetime > 0 {
		p := (float64(lifetime)/float64(g.config.FoodLifetime))*2.0 - 1.0
		return int(math.Round(float64(g.config.FoodValue) * p))
	}
	return g.config.FoodValue
}

func (g *Game) processCollisions() {
	deaths := make(map[int]bool)
	for id, s := range g.state.snakes {
		if s.IsDead() {
			continue
		}
		head := s.Head()

		// collisions with walls
		if head.X < 0 || head.X >= g.config.Width || head.Y < 0 || head.Y >= g.config.Height {
			deaths[id] = false
			continue
		}

		// collisions with snakes
		for oId, oS := range g.state.snakes {
			if oS.IsDead() {
				continue
			}

			for i, p := range oS.body {
				if id == oId && i == 0 {
					// skip snake's own head
					continue
				}
				if head == p {
					deaths[id] = id == oId
					continue
				}
			}
		}

		// collisions with food
		for p, l := range g.state.food {
			if head == p {
				s.length = max(0, s.length+g.getFoodValue(l))
				delete(g.state.food, p)

				if s.length == 0 {
					deaths[id] = false
				}
			}
		}
	}

	// kill all snakes that collided
	// this is done in a separate loop so that a snake that dies this turn still
	// acts as an obstacle to other snakes
	for id, suicide := range deaths {
		s := g.state.snakes[id]
		s.Kill(suicide)
	}
}

// IsGameOver implements pkg.Game.
func (g *Game) IsGameOver() bool {
	return g.state.currentTurn >= g.config.MaxTurns
}
