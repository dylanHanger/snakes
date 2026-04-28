# Configuration

You can change various configuration options by specifying them in a YAML file, which you can do with the `--config` command line argument.

```bash
snakes --config myconfig.yaml
```

## Example

This example matches the default settings.
Anything you don't specify will match these defaults (except for the players list)

```yaml
---
# Board size
width: 32
height: 32

# Game speed settings
turns: 1500
turnsPerSecond: 0 # 0 means as fast as possible

# The number of turns it takes to respawn
respawn: 10

# Seed the random number generator for repeatable experiments.
# If this is not specified, it will be different every time
seed: snake byte

# Food configuration
food:
  value: 5 # the length you gain if you eat fresh food
  lifetime: 0 # the number of turns food lives for, 0 means no rotting
  count: 1 # the number of foods on the board at any time

# Player configurations
players:
  - Random Randy:
      type: random # makes random moves

  - Keyboard Keagan:
      type: keyboard # controlled by a human player
      timeout: 250 # time in ms to submit a move
      keys:
        north: Up
        west: Left
        south: Down
        east: Right

  - External Eddie:
      type: custom # a hand-coded AI communicating over stdio
      silent: true # Prevent the output of chatty agents
      wait: true # Useful for slow snakes, or when you are debugging
      cmd: python
      args:
        - my_snake.py

  - Hard Harry:
      type: builtin # a built in basic AI to test yourself against
      difficulty: hard # easy/medium/hard
```

## Players

Players are listed under the `players` section in the configuration file.
Each entry has a name (the YAML key), and a `type` which determines how it makes its moves.
The supported types are `random`, `builtin`, `keyboard`, and `custom`.

These fields can be set on any player:
- **color**: the color of the snake on the board. Accepts hex strings (with or without a leading `#`), or Crayola color names. See [colors.md](colors.md) for more information.
- **silent**: if `true`, then chat messages from this player will be suppressed.
- **timeout**: the time (in milliseconds) that this player has to submit a move. If `0` or not specified, then the player must submit a move before the next turn starts.
- **wait**: if `true`, the game will pause on each turn until this player has submitted their move, overriding their timeout.

Some of the above will have no visible effect on some agents, for example `random` will never need to be waited for.

### `random`

Makes a random move every turn. You should be able to beat this with even the simplest strategies.

```yaml
- Random Randy:
    type: random
```

### `builtin`

A pre-programmed AI within the engine.
Can be configured to one of three difficulties; `easy`, `medium`, or `hard`.

```yaml
- Hard Harry:
    type: builtin
    difficulty: hard
```

### `keyboard`

Lets a human player control a snake from the keyboard.

```yaml
- Keyboard Keagan:
    type: keyboard
    timeout: 250 # I strongly recommend setting a long timeout or `wait: true` for keyboard players
    keys:
        north: Up
        west: Left
        south: Down
        east: Right
```

### `custom`

A snake controlled by an external program.
The engine spawns the program when the game starts, and communicates with it over standard I/O.
See [reference.md](reference.md) for more information.

```yaml
- External Eddie:
    type: custom
    cmd: python
    args:
      - my_snake.py
    timeout: 100
    wait: true
```

- **cmd**: the executable to run, resolved against your `PATH`. Either use a runtime (`python`, `node`, etc.) and pass your script as an argument, or use the path to a compiled executable directly.
- **args**: a list of arguments passed to `cmd`, such as a Python script or any flags your snake accepts
