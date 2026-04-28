# Reference

This document is the reference for how _Snakes!_ works: the rules of the game, and the protocol your custom agent uses to talk to the engine.
If you're new to the project, start with the README.

## Gameplay

The primary goal of the game is to become the longest snake. To do this, you must eat fresh food and avoid crashing. But remember, this is a multiplayer game, and the other snakes are trying to do the same thing.

Snakes are ranked by, in order:
1. Maximum length reached
2. Current length
3. Kills
4. Total deaths (fewer is better, and suicides count as two deaths)

### Food

There is always at least one food on the board at any time. If one is eaten, another will immediately spawn at a random position. Each turn, the food decays; becoming rotten and eventually disappearing.

Initially food is worth some value (5 by default). As it decays, this will decrease linearly. At the halfway point, the food becomes rotten, and eating it will shrink your snake! After eating the food, your snake will grow by one each turn for $v_t$ turns, where $v_t$ is the value of the food at that point in time.

```math
v_t = \lfloor v_0 \times (\frac{l_t}{l_0} \times 2 - 1) \rceil
```


### Kills and Deaths

When a snake attempts to move into a space that is occupied, the snake will die, and the occupant will be credited with a kill (unless it was a suicide).
If two snakes both attempt to move into the same space and collide head on, they are both credited with a kill.
If a snake attempts to move out of the play grid, it dies and nobody is credited with a kill.
If a snake's length shrinks below 1 (by eating rotten food), it will die.

## Creating a Custom Agent

Custom agents communicate with the game via standard I/O:

1. Game sends initial configuration and board state
2. Agent responds with a move direction (north/east/south/west)
3. Game sends updated state
4. Repeat until game over

See the `examples/` directory for sample implementations in various languages.

### Communication Protocol

When a custom agent starts, Snakes! writes four initial lines:

1. `<width> <height>`
2. `<food_lifetime> <food_value>`
3. `<player_count> <your_id>`
4. `<max_turns> <timeout_ms>`

If `wait: true` is set for your agent, then `<timeout_ms>` will be `-1`.

Each turn that your snake is alive, the engine sends:

- A line containing the food count: `<food_count>`
- One line per food item: `<lifetime_remaining> <x> <y>`
- One line per snake: `<id> <kills> <deaths> <current_length> <x1> <y1> … <xN> <yN>`

Dead snakes will have a length of 0 (and therefore no coordinates).

#### Moving

To submit a move, you must print a single line that starts with `n`, `e`, `s`, or `w` (case-insensitive).

Snakes cannot turn in the direction opposite to their current direction.
For example, if you are moving `North`, an output of `South` will be ignored, and you will continue moving `North`.
If you do not output a move within your allotted time, your snake will simply move forwards.

#### Logging

While the agent must output their move on `stdout`, `stderr` can be used to "talk".
Anything output on `stderr` gets displayed as a chat message.

### Coordinate System

All board coordinates use `(0,0)` in the top-left corner. `x` increases to the right, `y` increases as you move down the board.
