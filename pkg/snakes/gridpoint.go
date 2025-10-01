package snakes

import "math"

type GridPoint struct {
	X, Y int
}

// stupid
func abs(x int) int { return max(x, -x) }

func (p *GridPoint) Move(d Direction) GridPoint {
	dx, dy := d.Delta()
	return GridPoint{p.X + dx, p.Y + dy}
}

func (p *GridPoint) Reflect(w, h int) GridPoint {
	return GridPoint{w - p.X, h - p.Y}
}

// Towards returns the direction that moves towards another point
//
// When the other point is offset on both axes, Towards returns the direction of the greatest offset
func (p *GridPoint) Towards(other GridPoint) Direction {
	dx, dy := p.X-other.X, p.Y-other.Y

	// Equal points
	if dx == 0 && dy == 0 {
		return DirNone
	}

	// Pure horizontal movement
	if dy == 0 {
		if dx > 0 {
			return DirWest
		}
		return DirEast
	}

	// Pure vertical movement
	if dx == 0 {
		if dy > 0 {
			return DirNorth
		}
		return DirSouth
	}

	// Diagonal - determine principal direction by comparing absolute values
	if abs(dx) > abs(dy) {
		if dx > 0 {
			return DirWest
		}
		return DirEast
	}

	// Vertical component is larger or equal
	if dy > 0 {
		return DirNorth
	}
	return DirSouth
}

// AxesTowards returns the pair of directions that point towards another point
func (p *GridPoint) AxesTowards(other GridPoint) (Direction, Direction) {
	dx, dy := p.X-other.X, p.Y-other.Y
	var dh, dv Direction

	switch {
	case dx == 0:
		dh = DirNone
	case dx < 0:
		dh = DirEast
	case dx > 0:
		dh = DirWest
	}

	switch {
	case dy == 0:
		dv = DirNone
	case dy < 0:
		dv = DirSouth
	case dy > 0:
		dv = DirNorth
	}

	return dh, dv
}

// L1DistanceTo returns the Manhattan or L1 distance between two points
func (p *GridPoint) L1DistanceTo(other GridPoint) int {
	return abs(p.X-other.X) + abs(p.Y-other.Y)
}

// L2DistanceTo returns the Euclidean or L2 distance between two points
func (p *GridPoint) L2DistanceTo(other GridPoint) float64 {
	dx := p.X - other.X
	dy := p.Y - other.Y

	return math.Round(math.Sqrt(float64(dx*dx + dy*dy)))
}
