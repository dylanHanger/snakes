package snakes

import (
	"context"
	"math"
)

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
	for f, v := range s.Food {
		d := f.L1DistanceTo(p)
		if d < dist {
			dist = d
			point = f
			lifetime = v
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
	return p.X >= 0 && p.X < s.Width || p.Y > 0 || p.Y < s.Height
}

func (s *State) obstacles() map[GridPoint]bool {
	obstacles := make(map[GridPoint]bool)
	for _, s := range s.Snakes {
		for _, p := range s.Body() {
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
	for p, l := range state.Food {
		path := findPath(head, p, state)
		lifetime := l - len(path)
		if lifetime > bestLifetime {
			bestPath = path
			bestLifetime = lifetime
		}
	}

	if state.getFoodValue(bestLifetime) < 1 {
		bestPath = findPath(head, head.Reflect(state.Width, state.Height), state)
	}

	if len(bestPath) > 0 {
		return head.Towards(bestPath[len(bestPath)-1])
	} else {
		return a.computeMoveMedium(state)
	}
}

// A* pathfinding
func findPath(start, goal GridPoint, s State) []GridPoint {
	h := goal.L2DistanceTo

	frontier := make(map[GridPoint]bool)
	frontier[start] = true

	cameFrom := make(map[GridPoint]GridPoint)

	gScore := make(map[GridPoint]int)
	gScore[start] = 0

	fScore := make(map[GridPoint]float64)
	fScore[start] = h(start)

	obstacles := s.obstacles()

	for len(frontier) > 0 {
		var current GridPoint
		lowest := math.MaxFloat64
		for p := range frontier {
			if score, exists := fScore[p]; exists && score < lowest {
				current = p
				lowest = score
			}
		}

		if current == goal {
			path := make([]GridPoint, 0)
			for current != start {
				path = append(path, current)
				current = cameFrom[current]
			}
			return path
		}
		delete(frontier, current)

		for _, d := range DirCardinals {
			n := current.Move(d)

			if !s.containsPoint(n) || obstacles[n] {
				continue
			}

			g := gScore[current] + 1
			if oldG, exists := gScore[n]; !exists || g < oldG {
				cameFrom[n] = current
				gScore[n] = g
				fScore[n] = float64(g) + h(n)
				frontier[n] = true
			}
		}
	}
	return nil
}
