package snakes

type Snake struct {
	body   []GridPoint
	length int

	snakeScore

	respawnCounter int
	lastDirection  Direction

	player player
}

type snakeScore struct {
	Kills    int
	Deaths   int
	Suicides int

	MaxLength     int
	CurrentLength int
}

func (s *Snake) Score() snakeScore {
	return s.snakeScore
}

func (s *Snake) Kill(suicide bool) {
	s.lastDirection = DirNone
	s.body = []GridPoint{}

	s.Deaths += 1
	if suicide {
		s.Suicides += 1
	}
}

func (s *Snake) IsDead() bool {
	return len(s.body) == 0
}

func (s *Snake) Move(d Direction) {
	if s.IsDead() {
		return
	}
	head := s.Head()

	current := s.Direction()
	// reversing move (illegal, continue straight)
	if current != DirNone && current.Opposite() == d {
		d = current
	}

	// no move supplied
	if d == DirNone && current == DirNone {
		// default to a consistent direction until the agent issues commands
		d = DirCardinals[0]
	} else if d == DirNone {
		d = current
	}

	actualLength := min(len(s.body), s.length)
	s.body = append([]GridPoint{head.Move(d)}, s.body[:actualLength]...)

	s.CurrentLength = len(s.body)
	s.MaxLength = max(s.MaxLength, s.CurrentLength)

	s.lastDirection = d
}

func (s *Snake) Head() GridPoint {
	return s.body[0]
}

func (s *Snake) Tail() GridPoint {
	return s.body[len(s.body)-1]
}

func (s *Snake) Body() []GridPoint {
	return s.body
}

func (s *Snake) Direction() Direction {
	return s.lastDirection
}
