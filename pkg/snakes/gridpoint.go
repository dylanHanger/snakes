package snakes

type GridPoint struct {
	X, Y int
}

// stupid
func abs(x int) int { return max(x, -x) }

func (p *GridPoint) Move(d Direction) GridPoint {
	dx, dy := d.Delta()
	return GridPoint{p.X + dx, p.Y + dy}
}

// DirectionTo returns the direction that moves towards another point
//
// When the other point is offset on both axes, DirectionTo returns the direction of the greatest offset
func (p *GridPoint) DirectionTo(other GridPoint) Direction {
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

// DistanceTo returns the Manhattan or L1 distance between two points
func (p *GridPoint) DistanceTo(other GridPoint) int {
	return abs(p.X-other.X) + abs(p.Y-other.Y)
}
