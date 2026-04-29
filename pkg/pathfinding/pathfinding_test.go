package pathfinding

import (
	"math"
	"testing"
)

// point is a minimal grid coordinate used by these tests.
type point struct{ X, Y int }

func (p point) l1(q point) int {
	dx, dy := p.X-q.X, p.Y-q.Y
	if dx < 0 {
		dx = -dx
	}
	if dy < 0 {
		dy = -dy
	}
	return dx + dy
}

func (p point) l2(q point) float64 {
	dx, dy := float64(p.X-q.X), float64(p.Y-q.Y)
	return math.Sqrt(dx*dx + dy*dy)
}

func pointLess(a, b point) bool {
	if a.X != b.X {
		return a.X < b.X
	}
	return a.Y < b.Y
}

// grid models a rectangular board with a set of blocked cells. It exposes
// the neighbors callback expected by Find.
type grid struct {
	w, h      int
	blocked   map[point]bool
}

func newGrid(w, h int, blocked []point) grid {
	bs := make(map[point]bool, len(blocked))
	for _, p := range blocked {
		bs[p] = true
	}
	return grid{w: w, h: h, blocked: bs}
}

func (g grid) inBounds(p point) bool {
	return p.X >= 0 && p.X < g.w && p.Y >= 0 && p.Y < g.h
}

func (g grid) neighbors(p point) []point {
	deltas := [...]point{{0, -1}, {1, 0}, {0, 1}, {-1, 0}}
	ns := make([]point, 0, 4)
	for _, d := range deltas {
		n := point{p.X + d.X, p.Y + d.Y}
		if !g.inBounds(n) || g.blocked[n] {
			continue
		}
		ns = append(ns, n)
	}
	return ns
}

func (g grid) find(start, goal point) []point {
	heuristic := func(p point) float64 { return p.l2(goal) }
	return Find(start, goal, g.neighbors, heuristic, pointLess)
}

// pathIsValid checks that path is a contiguous L1 walk from start (exclusive)
// to goal (inclusive), stays in bounds, and avoids blocked cells.
func pathIsValid(t *testing.T, path []point, start, goal point, g grid) {
	t.Helper()
	if len(path) == 0 {
		t.Fatalf("empty path")
	}
	if path[0] != goal {
		t.Errorf("path[0] = %v, want goal %v", path[0], goal)
	}
	prev := start
	// path is reversed (goal first, neighbour-of-start last).
	for i := len(path) - 1; i >= 0; i-- {
		p := path[i]
		if !g.inBounds(p) {
			t.Errorf("path step %v out of bounds", p)
		}
		if g.blocked[p] {
			t.Errorf("path step %v lands on blocked cell", p)
		}
		if prev.l1(p) != 1 {
			t.Errorf("non-adjacent step: %v -> %v", prev, p)
		}
		prev = p
	}
}

func TestFind_StraightLine(t *testing.T) {
	g := newGrid(10, 10, nil)
	start, goal := point{0, 0}, point{5, 0}

	path := g.find(start, goal)
	if len(path) != 5 {
		t.Fatalf("len(path) = %d, want 5", len(path))
	}
	pathIsValid(t, path, start, goal, g)
}

func TestFind_LShape(t *testing.T) {
	g := newGrid(10, 10, nil)
	start, goal := point{0, 0}, point{3, 4}

	path := g.find(start, goal)
	// L1 distance = 7, so any optimal path has 7 steps.
	if len(path) != 7 {
		t.Fatalf("len(path) = %d, want 7 (L1 distance)", len(path))
	}
	pathIsValid(t, path, start, goal, g)
}

func TestFind_StartEqualsGoal(t *testing.T) {
	g := newGrid(10, 10, nil)
	p := point{3, 3}
	path := g.find(p, p)
	if len(path) != 0 {
		t.Errorf("len(path) = %d, want 0 for start==goal", len(path))
	}
}

func TestFind_AroundObstacle(t *testing.T) {
	// Vertical wall down the middle with one gap at y=4.
	var wall []point
	for y := range 10 {
		if y == 4 {
			continue
		}
		wall = append(wall, point{X: 3, Y: y})
	}
	g := newGrid(10, 10, wall)

	start, goal := point{0, 0}, point{6, 0}

	path := g.find(start, goal)
	if path == nil {
		t.Fatalf("path is nil but route exists")
	}
	pathIsValid(t, path, start, goal, g)

	// Direct L1 distance is 6, but the wall forces a detour through y=4.
	if len(path) <= 6 {
		t.Errorf("path length %d does not reflect the detour", len(path))
	}
}

func TestFind_Unreachable(t *testing.T) {
	// Box the goal in completely.
	goal := point{5, 5}
	wall := []point{
		{X: 4, Y: 5}, {X: 6, Y: 5},
		{X: 5, Y: 4}, {X: 5, Y: 6},
	}
	g := newGrid(10, 10, wall)

	path := g.find(point{0, 0}, goal)
	if path != nil {
		t.Errorf("expected nil path for unreachable goal, got %v", path)
	}
}

func TestFind_GoalOutOfBounds(t *testing.T) {
	g := newGrid(10, 10, nil)
	path := g.find(point{0, 0}, point{20, 20})
	if path != nil {
		t.Errorf("expected nil path for out-of-bounds goal, got %v", path)
	}
}

func TestFind_Deterministic(t *testing.T) {
	// Open grid: many equal-cost paths exist between start and goal,
	// which is exactly the scenario where map iteration used to leak
	// non-determinism into the result.
	g := newGrid(20, 20, nil)
	start, goal := point{1, 1}, point{15, 12}

	first := g.find(start, goal)
	if first == nil {
		t.Fatalf("path is nil")
	}

	for i := range 50 {
		got := g.find(start, goal)
		if len(got) != len(first) {
			t.Fatalf("run %d: len mismatch: got %d, want %d", i, len(got), len(first))
		}
		for j := range got {
			if got[j] != first[j] {
				t.Fatalf("run %d: step %d differs: got %v, want %v", i, j, got[j], first[j])
			}
		}
	}
}

func TestFind_OptimalAroundDiagonalObstacles(t *testing.T) {
	// Scatter a few obstacles that don't block any optimal path; the
	// returned path must still be of L1 length.
	blocked := []point{
		{X: 2, Y: 5}, {X: 5, Y: 2}, {X: 7, Y: 7},
	}
	g := newGrid(15, 15, blocked)

	start, goal := point{0, 0}, point{10, 10}

	path := g.find(start, goal)
	if path == nil {
		t.Fatalf("path is nil")
	}
	pathIsValid(t, path, start, goal, g)
	if len(path) != 20 {
		t.Errorf("len(path) = %d, want 20 (L1 distance)", len(path))
	}
}
