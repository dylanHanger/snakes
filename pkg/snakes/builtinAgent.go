package snakes

import (
	"context"
	"math"
	"slices"
	"sort"
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

func (a *builtInAgent) computeMove(state State) Direction {
	me := state.Snakes[state.Id]
	if me.IsDead() {
		// Safe fallback if we're dead or our snake doesn't exist in the state
		// FIXME: Find whatever is causing dead agents to be asked for moves
		return DirNone
	}

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
	myHead := me.Head()

	var closest GridPoint
	closestDist := math.MaxInt
	for p := range state.Food {
		dist := p.DistanceTo(myHead)
		if dist < closestDist {
			closestDist = dist
			closest = p
		}
	}

	currentDir := me.Direction()
	targetDir := myHead.DirectionTo(closest)
	if currentDir == targetDir.Opposite() {
		targetDir = targetDir.Next()
	}
	return targetDir
}

func (a *builtInAgent) computeMoveMedium(state State) Direction {
	me := state.Snakes[state.Id]
	myHead := me.Head()
	currentDir := me.Direction()
	w, h := state.Width, state.Height

	// 1. Create obstacles map including walls
	obstacles := make(map[GridPoint]bool)
	for _, s := range state.Snakes {
		for _, p := range s.Body() {
			obstacles[p] = true
		}
	}
	for x := range w {
		obstacles[GridPoint{X: x, Y: -1}] = true
		obstacles[GridPoint{X: x, Y: h}] = true
	}
	for y := range h {
		obstacles[GridPoint{X: -1, Y: y}] = true
		obstacles[GridPoint{X: w, Y: y}] = true
	}

	// 2. Evaluate food - consider both value and reachability
	rotPoint := state.FoodLifetime / 2

	var bestFood GridPoint
	bestScore := -math.MaxInt
	for p, v := range state.Food {
		dist := p.DistanceTo(myHead)
		// Skip if it will be rotten by the time we get there
		vt := v - dist
		if vt < rotPoint {
			continue
		}

		if vt > bestScore {
			bestScore = vt
			bestFood = p
		}
	}

	// 3. If no good food found, follow tail for safety
	if bestScore == -math.MaxInt {
		tail := me.Body()[len(me.Body())-1]
		bestFood = tail
	}

	// 4. What direction do I want to move?
	targetDir := myHead.DirectionTo(bestFood)

	// 5. Safety check with proper rotation tracking
	safeDir := targetDir
	checkedDirs := 0
	maxDirs := 4 // Total possible directions

	// Try directions in order of preference: target, next, previous, opposite
	preferences := []Direction{
		targetDir,
		targetDir.Next(),
		targetDir.Previous(),
		targetDir.Opposite(),
	}

	for _, dir := range preferences {
		nextHead := myHead.Move(dir)

		// Check if move is safe (not hitting wall or snake)
		safe := nextHead.X >= 0 && nextHead.Y >= 0 && nextHead.X < w && nextHead.Y < h && !obstacles[nextHead]

		// Also prevent reversing
		if dir == currentDir.Opposite() {
			safe = false
		}

		if safe {
			safeDir = dir
			break
		}

		checkedDirs++
		if checkedDirs >= maxDirs {
			// If no safe option, use the target direction (we're going to die anyway)
			safeDir = targetDir
			break
		}
	}

	return safeDir
}

func (a *builtInAgent) computeMoveHard(state State) Direction {
	me := state.Snakes[state.Id]
	myHead := me.Head()
	myTail := me.Tail()
	w, h := state.Width, state.Height

	// 1. Create obstacles map including walls
	obstacles := make(map[GridPoint]bool)
	for _, s := range state.Snakes {
		for _, p := range s.Body() {
			obstacles[p] = true
		}
	}
	for x := range w {
		obstacles[GridPoint{X: x, Y: -1}] = true
		obstacles[GridPoint{X: x, Y: h}] = true
	}
	for y := range h {
		obstacles[GridPoint{X: -1, Y: y}] = true
		obstacles[GridPoint{X: w, Y: y}] = true
	}

	dangerZones := make(map[GridPoint]bool)
	for sId, s := range state.Snakes {
		if sId == state.Id || s.IsDead() {
			continue
		}
		p := s.Head()
		for _, d := range DirCardinals {
			n := p.Move(d)
			if !obstacles[n] {
				dangerZones[n] = true
			}
		}
	}

	// 2. Evaluate food - consider both value and reachability
	rotPoint := state.FoodLifetime / 2

	bestPath := findPath(myHead, myTail, obstacles, w, h)
	bestScore := 0
	for p, v := range state.Food {
		path := findPath(myHead, p, obstacles, w, h)
		if path == nil {
			continue // unreachable, no loss
		}

		// Skip if it will be rotten by the time we get there
		dist := len(path)
		vt := v - dist
		if vt < rotPoint {
			continue
		}

		if vt > bestScore {
			// its worth going for, but can I guarantee I get it?
			guaranteed := true
			for sId, s := range state.Snakes {
				if state.Id == sId || s.IsDead() {
					continue
				}
				otherHead := s.Head()
				otherPath := findPath(otherHead, p, obstacles, w, h)
				if otherPath != nil && len(otherPath) <= len(path) {
					guaranteed = false
					break
				}
			}

			if !guaranteed {
				continue
			}

			bestScore = vt
			bestPath = path
		}
	}

	var targetDir Direction
	if len(bestPath) > 0 {
		targetDir = myHead.DirectionTo(bestPath[0])
	}

	safeDirections := []Direction{}
	unsafeDirections := []Direction{}
	currentDir := me.Direction()

	// Calculate board center
	centerX := w / 2
	centerY := h / 2
	centerPoint := GridPoint{X: centerX, Y: centerY}

	// Map to store distances to center for each direction
	centerDistances := make(map[Direction]int)

	for _, d := range DirCardinals {
		if d == currentDir.Opposite() {
			continue // no reversing
		}

		nextPos := myHead.Move(d)
		if nextPos.X >= 0 && nextPos.X < w && nextPos.Y >= 0 && nextPos.Y < h && !obstacles[nextPos] {
			dangerous := dangerZones[nextPos]

			// Calculate distance to center
			distToCenter := nextPos.DistanceTo(centerPoint)
			centerDistances[d] = distToCenter

			if !dangerous {
				safeDirections = append(safeDirections, d)
			} else {
				unsafeDirections = append(unsafeDirections, d)
			}
		}
	}

	if slices.Contains(safeDirections, targetDir) {
		return targetDir
	} else if len(safeDirections) > 0 {

		// Sort safe directions by distance to center
		sort.Slice(safeDirections, func(i, j int) bool {
			return centerDistances[safeDirections[i]] < centerDistances[safeDirections[j]]
		})

		// Return the direction that gets us closest to center
		return safeDirections[0]
	} else if len(unsafeDirections) > 0 {
		// Sort unsafe directions by distance to center too
		sort.Slice(unsafeDirections, func(i, j int) bool {
			return centerDistances[unsafeDirections[i]] < centerDistances[unsafeDirections[j]]
		})

		return unsafeDirections[0]
	}

	r := a.random
	if r != nil {
		return DirCardinals[r.IntN(len(DirCardinals))]
	}
	return DirCardinals[0]
}

// A* pathfinding
func findPath(start, goal GridPoint, obstacles map[GridPoint]bool, width, height int) []GridPoint {
	frontier := make(map[GridPoint]bool)
	frontier[start] = true

	cameFrom := make(map[GridPoint]GridPoint)

	g := make(map[GridPoint]int)
	g[start] = 0

	f := make(map[GridPoint]int)
	f[start] = 0

	for len(frontier) > 0 {
		var current GridPoint
		lowest := math.MaxInt

		for p := range frontier {
			if score, exists := f[p]; exists && score < lowest {
				lowest = score
				current = p
			}
		}

		if current == goal {
			path := []GridPoint{}
			for current != start {
				path = append([]GridPoint{current}, path...)
				current = cameFrom[current]
			}
			return path
		}
		delete(frontier, current)

		for _, d := range DirCardinals {
			n := current.Move(d)

			if n.X < 0 || n.X >= width || n.Y < 0 || n.Y >= height || obstacles[n] {
				continue
			}

			tentativeG := g[current] + 1
			if gScore, inMap := g[n]; !inMap || gScore > tentativeG {
				cameFrom[n] = current
				g[n] = tentativeG
				f[n] = tentativeG + n.DistanceTo(goal)

				frontier[n] = true
			}
		}
	}

	return nil
}
