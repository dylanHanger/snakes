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
width: 20 # Board width
height: 20 # Board height
turns: 1500 # Maximum game turns
turnsPerSecond: 10 # Game speed

respawn: 10 # Turns to wait before respawning

food:
  value: 5 # Base value of food
  lifetime: 100 # How long food remains (0 = forever)
  count: 1 # Number of food items on the board

players:
  - MyAgent:
      type: custom
      executable: /path/to/agent
      arguments:
        - arg0
        - arg1
      timeout: 250 # Milliseconds to wait for move
      waitFor: false # Whether to block waiting for move
```

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

## Why Go?

I wanted to learn Go, and the multi-threaded nature of Snakes! made it a perfect place to start.

In addition to that, the original Rust code was way too complicated, since I made it while still learning both Rust and Bevy.
Porting to Go gave me the opportunity to clean up the architecture and turn Snakes! into a much more thought out framework.
You can expect to see some very cool things in the future!

## License

[MIT License](LICENSE)

