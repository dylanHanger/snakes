// Package pathfinding implements A* search. It is generic over the node
// type so callers don't need to commit to a specific grid representation.
package pathfinding

import "container/heap"

// Find runs A* from start to goal. It returns the path in reverse order
// (goal first, neighbour-of-start last), or nil if no path exists.
//
// neighbors enumerates the passable successors of a node. heuristic is an
// admissible (never-overestimating) cost estimate from a node to the goal;
// for grid maps with unit-cost moves, Euclidean distance to the goal is a
// safe choice. less is a strict total order on P used to break ties on
// equal fScore so results are deterministic across runs.
func Find[P comparable](
	start, goal P,
	neighbors func(P) []P,
	heuristic func(P) float64,
	less func(P, P) bool,
) []P {
	cameFrom := make(map[P]P)

	gScore := make(map[P]int)
	gScore[start] = 0

	fScore := make(map[P]float64)
	fScore[start] = heuristic(start)

	frontier := &pq[P]{less: less, items: []item[P]{{point: start, f: fScore[start]}}}
	heap.Init(frontier)

	for frontier.Len() > 0 {
		current := heap.Pop(frontier).(item[P]).point

		if current == goal {
			path := make([]P, 0)
			for current != start {
				path = append(path, current)
				current = cameFrom[current]
			}
			return path
		}

		for _, n := range neighbors(current) {
			g := gScore[current] + 1
			if oldG, exists := gScore[n]; !exists || g < oldG {
				cameFrom[n] = current
				gScore[n] = g
				f := float64(g) + heuristic(n)
				fScore[n] = f
				heap.Push(frontier, item[P]{point: n, f: f})
			}
		}
	}
	return nil
}

// item is a frontier entry: a node and its current fScore.
type item[P comparable] struct {
	point P
	f     float64
}

// pq is a min-heap of items keyed on (f, less(point, point)).
type pq[P comparable] struct {
	less  func(P, P) bool
	items []item[P]
}

func (q pq[P]) Len() int { return len(q.items) }
func (q pq[P]) Less(i, j int) bool {
	a, b := q.items[i], q.items[j]
	if a.f != b.f {
		return a.f < b.f
	}
	return q.less(a.point, b.point)
}
func (q pq[P]) Swap(i, j int)   { q.items[i], q.items[j] = q.items[j], q.items[i] }
func (q *pq[P]) Push(x any)     { q.items = append(q.items, x.(item[P])) }
func (q *pq[P]) Pop() any       { n := len(q.items); x := q.items[n-1]; q.items = q.items[:n-1]; return x }
