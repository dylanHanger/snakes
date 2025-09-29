# Snakes 🐍

A Go implementation of the Snakes game framework, an evolution of the [original Rust version](https://github.com/dylanHanger/snakes).

## Overview

Snakes is a competitive programming environment where multiple snake agents compete on a grid.
The goal is to become the longest snake by eating apples while avoiding collisions with walls, other snakes, or your own body.

### Game Mechanics

- **Objective**: Become the longest snake by eating apples and avoiding crashes
- **Movement**: Snakes move one space per turn in one of four directions (north, east, south, west)
- **Food**: Apples appear on the board that increase (or decrease!) your snake's length when eaten
- **Death**: Snakes die by colliding with walls, other snakes, or themselves
- **Scoring**: Snakes are ranked by maximum length achieved, kills, deaths, and current length

## Usage

You can download a release build and run it like any other application.
You can run snakes by cloning this repo and then running `go run cmd/snakes`

The game loads configuration from `config/config.yaml` by default, you can change this by providing the `-config FILE` command line argument

## Configuration

Configure your game through the `config.yaml` file:

```yaml
width: 32 # Board width
height: 32 # Board height
turns: 1500 # Maximum game turns
turnsPerSecond: 0 # 0 = run as fast as possible, >0 locks to that TPS

respawn: 2 # Turns you miss while dead (0 = spawn next turn)

food:
  value: 5 # Base value of food
  lifetime: 50 # How long food remains (0 = forever)
  count: 1 # Number of food items on the board

players:
  - Ferris:
      type: custom
      cmd: examples/ferris.exe
      args:
        - --difficulty
        - hard
      timeout: 250 # Per-player move budget in milliseconds
      wait: false # If true, engine waits indefinitely for a move
  - EasyBot:
      type: builtin
      difficulty: easy
      color: bluetiful
      timeout: 0 # 0 = use global turn duration
```

Key notes:

- `turnsPerSecond` controls how quickly the game clock advances. Setting it to `0` tells the engine to process turns as soon as inputs arrive.
- Each player can override their own `timeout` (milliseconds) and `wait` behaviour. When `timeout` is `0`, the engine uses the global turn duration; with `wait: true`, the engine blocks until the agent replies.
- Custom agents use `cmd` plus optional `args` instead of the older `executable` / `arguments` keys.
- Optional fields such as `color` (hex or Crayola name) and `silent` still work as before.

## Agent Types

- **Custom**: External program that communicates via standard I/O
- **Builtin**: Pre-programmed AI with difficulty levels (easy/medium/hard)
- **Random**: Makes random moves
- **Keyboard**: Human-controlled

## Creating a Custom Agent

Custom agents communicate with the game via standard I/O:

1. Game sends initial configuration and board state
2. Agent responds with a move direction (north/east/south/west)
3. Game sends updated state
4. Repeat until game over

See the `examples/` directory for sample implementations.

### External Agent Protocol

When a custom agent starts, Snakes! writes four initial lines:

1. `<width> <height>`
2. `<food_lifetime> <food_value>`
3. `<player_count> <your_id>`
4. `<max_turns> <timeout_ms>` (`-1` if `wait: true`)

Each turn that the agent is alive, the engine sends:

- A line containing the food count: `<apple_count>`
- One line per food item: `<lifetime_remaining> <x> <y>`
- One line per snake: `<id> <kills> <deaths> <segment_count> <x1> <y1> … <xN> <yN>`

Clients should reply with a single line that starts with `n`, `e`, `s`, or `w` (case-insensitive).

### Coordinate System

All board coordinates use `(0,0)` in the top-left corner. `x` increases to the right, `y` increases as you move down the board.

## Differences from Rust Version (v1)

### New Features

1. **Flexible Food Configuration**:

   - `food.count`: Configure multiple food items on the board
   - `food.lifetime`: Set to 0 to disable food decay

2. **Per-Player Settings**:

   - Individual `timeout` for each player

3. **Game Speed Control**:

   - `turnsPerSecond` directly controls game speed
   - No separate "Quick" mode - automatic based on timeouts and turn duration

4. **Agent Communication**:

   - Direction input must be text starting with 'n'/'e'/'s'/'w'
   - No support for 0/1/2/3 numeric direction input

5. **Improved Interface**:

   - Support for custom snake colors (hex, crayola names)
   - Support for custom skins (coming soon!)

### Identical Mechanics

- Core gameplay (movement, collisions, growth)
- Scoring system and priorities
- Agent implementation interface
- Food decay mechanic (when enabled)
  - Although you should be aware of multiple food items, as well as no-decay food

## License

[MIT License](LICENSE)
