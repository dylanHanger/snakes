package snakes

type State struct {
	*Config

	Id int
	CurrentTurn int

	Snakes []Snake // TODO: different representation for agents (support invisibility, etc)
	Food   map[GridPoint]int
}
