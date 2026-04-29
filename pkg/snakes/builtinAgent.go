package snakes

import (
	"context"
	"math"
	"sort"

	"github.com/dylanHanger/snakes/pkg/pathfinding"
)

// gridPointLess gives a stable ordering on GridPoint so map-iteration ties
// resolve the same way every run.
func gridPointLess(a, b GridPoint) bool {
	if a.X != b.X {
		return a.X < b.X
	}
	return a.Y < b.Y
}

// sortedFoodKeys returns food positions in a deterministic order.
func sortedFoodKeys(food map[GridPoint]int) []GridPoint {
	keys := make([]GridPoint, 0, len(food))
	for p := range food {
		keys = append(keys, p)
	}
	sort.Slice(keys, func(i, j int) bool { return gridPointLess(keys[i], keys[j]) })
	return keys
}

type BuiltInDifficulty string

const (
	Easy   BuiltInDifficulty = "easy"
	Medium BuiltInDifficulty = "medium"
	Hard   BuiltInDifficulty = "hard"
)

type builtInAgent struct {
	baseAgent
	difficulty BuiltInDifficulty
}

func NewBuiltInAgent(difficulty BuiltInDifficulty) *builtInAgent {
	return &builtInAgent{
		difficulty: difficulty,
	}
}

func (a *builtInAgent) Stop(ctx context.Context) error { return nil }

func (a *builtInAgent) Send(state State, ctx context.Context) (<-chan Direction, error) {
	replyChan := make(chan Direction, 1)
	defer close(replyChan)

	move := a.computeMove(state)
	replyChan <- move

	return replyChan, nil
}

func (s *State) findClosestFood(p GridPoint) (GridPoint, int) {
	var point GridPoint
	var lifetime int
	dist := math.MaxInt
	for _, f := range sortedFoodKeys(s.Food) {
		d := f.L1DistanceTo(p)
		if d < dist {
			dist = d
			point = f
			lifetime = s.Food[f]
		}
	}

	return point, lifetime
}

func (s *State) getFoodValue(l int) int {
	if s.FoodLifetime > 0 {
		p := (float64(l)/float64(s.FoodLifetime))*2.0 - 1.0
		return int(math.Round(float64(s.FoodValue) * p))
	}
	return s.FoodValue
}

func (s *State) containsPoint(p GridPoint) bool {
	return p.X >= 0 && p.X < s.Width && p.Y >= 0 && p.Y < s.Height
}

func (s *State) obstacles() map[GridPoint]bool {
	obstacles := make(map[GridPoint]bool)
	for _, snake := range s.Snakes {
		for _, p := range snake.Body() {
			obstacles[p] = true
		}
	}
	w, h := s.Width, s.Height
	for x := range w {
		obstacles[GridPoint{X: x, Y: -1}] = true
		obstacles[GridPoint{X: x, Y: h}] = true
	}
	for y := range h {
		obstacles[GridPoint{X: -1, Y: y}] = true
		obstacles[GridPoint{X: w, Y: y}] = true
	}

	return obstacles
}

// findPath wraps the generic A* in pkg/pathfinding for the snakes grid.
func (s *State) findPath(start, goal GridPoint) []GridPoint {
	obstacles := s.obstacles()

	neighbors := func(p GridPoint) []GridPoint {
		ns := make([]GridPoint, 0, len(DirCardinals))
		for _, d := range DirCardinals {
			n := p.Move(d)
			if !s.containsPoint(n) || obstacles[n] {
				continue
			}
			ns = append(ns, n)
		}
		return ns
	}

	heuristic := func(p GridPoint) float64 { return p.L2DistanceTo(goal) }

	return pathfinding.Find(start, goal, neighbors, heuristic, gridPointLess)
}

func (a *builtInAgent) computeMove(state State) Direction {
	switch a.difficulty {
	case Easy:
		return a.computeMoveEasy(state)
	case Medium:
		return a.computeMoveMedium(state)
	case Hard:
		return a.computeMoveHard(state)
	default:
		panic("unexpected difficulty")
	}
}

func (a *builtInAgent) computeMoveEasy(state State) Direction {
	me := state.Snakes[state.Id]
	if me.IsDead() {
		return DirNone
	}
	head := me.Head()
	direction := me.Direction()
	center := GridPoint{X: state.Width / 2, Y: state.Height / 2}

	closest, _ := state.findClosestFood(head)

	dx, dy := head.AxesTowards(closest)
	if dx == DirNone {
		if dy == DirNone {
			return DirNone
		} else if dy == direction.Opposite() {
			_, dy := head.AxesTowards(center)
			return dy
		}
		return dy
	} else if dx == direction.Opposite() {
		if dy == DirNone {
			dx, _ := head.AxesTowards(center)
			return dx
		}
	}
	return dx
}

func (a *builtInAgent) computeMoveMedium(state State) Direction {
	me := state.Snakes[state.Id]
	if me.IsDead() {
		return DirNone
	}
	head := me.Head()
	direction := me.Direction()
	center := GridPoint{state.Width / 2, state.Height / 2}

	closest, _ := state.findClosestFood(head)

	dx, dy := head.AxesTowards(closest)
	var best Direction
	if dx == DirNone {
		if dy == DirNone {
			best = DirNone
		} else if dy == direction.Opposite() {
			_, best = head.AxesTowards(center)
		} else {
			best = dy
		}
	} else if dx == direction.Opposite() {
		if dy == DirNone {
			best, _ = head.AxesTowards(center)
		} else {
			best = dx
		}
	} else {
		best = dx
	}

	next := head.Move(best)
	obstacles := state.obstacles()

	for range 3 {
		if state.containsPoint(next) && !obstacles[next] {
			break
		}
		best = best.Next()
		next = head.Move(best)
	}

	return best
}

func (a *builtInAgent) computeMoveHard(state State) Direction {
	me := state.Snakes[state.Id]
	if me.IsDead() {
		return DirNone
	}
	head := me.Head()

	var bestPath []GridPoint
	bestLifetime := math.MinInt
	for _, p := range sortedFoodKeys(state.Food) {
		l := state.Food[p]
		path := state.findPath(head, p)

		if path == nil {
			continue
		}

		lifetime := l - len(path)
		if lifetime > bestLifetime {
			bestPath = path
			bestLifetime = lifetime
		}
	}

	if bestPath == nil || state.getFoodValue(bestLifetime) < 1 {
		bestPath = state.findPath(head, head.Reflect(state.Width, state.Height))
	}

	if len(bestPath) > 0 {
		return head.Towards(bestPath[len(bestPath)-1])
	} else {
		return a.computeMoveMedium(state)
	}
}
