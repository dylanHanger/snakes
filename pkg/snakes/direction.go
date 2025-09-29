package snakes

import (
	"strings"
)

type Direction int

const (
	DirNone Direction = iota
	DirNorth
	DirEast
	DirSouth
	DirWest
)

var DirCardinals = [4]Direction{DirNorth, DirEast, DirSouth, DirWest}

func (d Direction) String() string {
	switch d {
	case DirNorth:
		return "north"
	case DirEast:
		return "east"
	case DirSouth:
		return "south"
	case DirWest:
		return "west"
	default:
		return "none"
	}
}

func (d Direction) Previous() Direction {
	switch d {
	case DirNorth:
		return DirWest
	case DirEast:
		return DirNorth
	case DirSouth:
		return DirEast
	case DirWest:
		return DirSouth
	default:
		return DirNone
	}
}

func (d Direction) Next() Direction {
	switch d {
	case DirNorth:
		return DirEast
	case DirEast:
		return DirSouth
	case DirSouth:
		return DirWest
	case DirWest:
		return DirNorth
	default:
		return DirNone
	}
}

func (d Direction) Opposite() Direction {
	switch d {
	case DirNorth:
		return DirSouth
	case DirSouth:
		return DirNorth
	case DirEast:
		return DirWest
	case DirWest:
		return DirEast
	default:
		return DirNone
	}
}

func (d Direction) Delta() (x, y int) {
	switch d {
	case DirNorth:
		return 0, -1
	case DirSouth:
		return 0, 1
	case DirEast:
		return 1, 0
	case DirWest:
		return -1, 0
	default:
		return 0, 0
	}
}

func ParseDirection(s string) Direction {
	if len(s) == 0 {
		return DirNone
	}

	switch strings.ToLower(s)[0] {
	case 'n':
		return DirNorth
	case 's':
		return DirSouth
	case 'e':
		return DirEast
	case 'w':
		return DirWest
	default:
		return DirNone
	}
}
