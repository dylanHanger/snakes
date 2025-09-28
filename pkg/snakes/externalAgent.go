package snakes

import (
	"bufio"
	"context"
	"errors"
	"fmt"
	"io"
	"os/exec"
)

type externalAgent struct {
	baseAgent

	cmd    *exec.Cmd
	stdin  io.WriteCloser
	stdout io.ReadCloser
	stderr io.ReadCloser

	executable string
	args       []string

	exited chan struct{}

	initialized bool
}

func NewExternal(executable string, args ...string) *externalAgent {
	return &externalAgent{
		executable: executable,
		args:       args,
	}
}

func (a *externalAgent) Start(ctx context.Context) error {
	a.initialized = false // just in case we are starting the same agent again
	a.ctx = ctx

	a.cmd = exec.Command(a.executable, a.args...)

	var err error
	a.stdin, err = a.cmd.StdinPipe()
	if err != nil {
		return fmt.Errorf("failed to create stdin pipe: %w", err)
	}

	a.stdout, err = a.cmd.StdoutPipe()
	if err != nil {
		return fmt.Errorf("failed to create stdout pipe: %w", err)
	}

	a.stderr, err = a.cmd.StderrPipe()
	if err != nil {
		return fmt.Errorf("failed to create stderr pipe: %w", err)
	}

	if err := a.cmd.Start(); err != nil {
		return fmt.Errorf("failed to start agent: %w", err)
	}

	a.exited = make(chan struct{})
	go func() {
		if a.cmd.Process != nil {
			a.cmd.Wait()
		}
		close(a.exited)
	}()

	return nil
}

func (a *externalAgent) Stop(ctx context.Context) error {
	a.stdin.Close()

	select {
	case <-a.exited:
	case <-ctx.Done():
		if a.cmd.Process != nil {
			err := a.cmd.Process.Kill()
			return errors.Join(err, ctx.Err())
		}
	}
	return nil
}

func (a *externalAgent) Send(state State, context context.Context) (<-chan Direction, error) {
	replyChan := make(chan Direction, 1)

	width, height := state.Width, state.Height
	foodLifetime, foodValue := state.FoodLifetime, state.FoodValue
	maxTurns := state.MaxTurns
	// respawn := state.RespawnTime // TODO: include respawn time in the config?

	timeout := state.Players[state.Id].Timeout().Milliseconds()
	wait := state.Players[state.Id].WaitFor()
	if wait {
		timeout = -1
	}

	numPlayers, playerId := len(state.Players), state.Id

	// First-time initialization message (should only be sent once)
	if !a.initialized {
		// <arena width> <arena height>
		// <food lifetime> <food value>
		// <number of players> <your id>
		// <number of turns> <timeout>
		_, err := fmt.Fprintf(a.stdin, "%d %d\n%d %d\n%d %d\n%d %d\n",
			width, height,
			foodLifetime, foodValue,
			numPlayers, playerId,
			maxTurns, timeout)
		if err != nil {
			return nil, fmt.Errorf("failed to write initialization to agent: %w", err)
		}
		a.initialized = true
	}

	// Current turn state information
	// Get food information
	food := state.Food

	// Format: <number of apples>
	_, err := fmt.Fprintf(a.stdin, "%d\n", len(food))
	if err != nil {
		return nil, fmt.Errorf("failed to write number of apples: %w", err)
	}

	// For each food item: <lifetime> <x> <y>
	for point, lifetime := range food {
		_, err := fmt.Fprintf(a.stdin, "%d %d %d\n", lifetime, point.X, height-point.Y-1)
		if err != nil {
			return nil, fmt.Errorf("failed to write food info: %w", err)
		}
	}

	// Snake information
	for id, snake := range state.Snakes {
		// Format: <id> <kills> <deaths> <length> <x1 y1 x2 y2 x3 y3 ...>
		body := snake.Body()
		score := snake.Score()
		_, err := fmt.Fprintf(a.stdin, "%d %d %d %d",
			id, score.Kills, score.Deaths, len(body))
		if err != nil {
			return nil, fmt.Errorf("failed to write snake info: %w", err)
		}

		// Add snake body coordinates
		for _, point := range body {
			_, err := fmt.Fprintf(a.stdin, " %d %d", point.X, height-point.Y-1)
			if err != nil {
				return nil, fmt.Errorf("failed to write snake body: %w", err)
			}
		}

		// End this snake's line
		_, err = fmt.Fprintln(a.stdin)
		if err != nil {
			return nil, fmt.Errorf("failed to finish snake line: %w", err)
		}
	}

	go func() {
		defer close(replyChan)
		scanner := bufio.NewScanner(a.stdout)
		if scanner.Scan() {
			replyChan <- ParseDirection(scanner.Text())
		}
	}()

	return replyChan, nil
}

func (a *externalAgent) Talk(ctx context.Context) <-chan string {
	if a.talk == nil {
		a.talk = make(chan string)
		go func() {
			defer close(a.talk)
			scanner := bufio.NewScanner(a.stderr)
			for scanner.Scan() {
				select {
				case a.talk <- scanner.Text():
				case <-ctx.Done():
					return
				}
			}
		}()
	}
	return a.talk
}
