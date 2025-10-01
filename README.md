# Snakes!

[![](https://dcbadge.limes.pink/api/server/zapH4Sz7wH)](https://discord.gg/zapH4Sz7wH)

"Snake" is a classic arcade game where the player controls a snake which gets longer every time it eats. If the snake collides with anything, including itself (which is more likely to happen as it gets longer!), it will die and its length will be reset.
"Snakes!" is an AI competition, a coding challenge in which participants must write a program that will play a multiplayer version of Snake against other snake AIs. The snake that holds the record for the longest length throughout the course of the game is the winner!

## Getting Started

You can download a release build and run it like any other application.
If you are so inclined, you can clone this repo and build it yourself.

```
git clone https://github.com/dylanHanger/snakes
cd snakes
go run ./cmd/snakes --config config/config.yaml
```

The game loads configuration from `config.yaml` by default, you can change this by providing the `--config FILE` command line argument.

## Gameplay

The primary goal of the game is to become the longest snake. To do this, you must eat fresh food and avoid crashing. But remember, this is a multiplayer game, and the other snakes are trying to do the same thing.

### Food

There is always at least one food on the board at any time. If one is eaten, another will immediately spawn at a random position. Each turn, the food decays; becoming rotten and eventually disappearing.

Initially food is worth some value (5 by default). As it decays, this will decrease linearly. At the halfway point, the food becomes rotten, and eating it will shrink your snake! After eating the food, your snake will grow by one each turn for $v_t$ turns, where $v_t$ is the value of the food at that point in time.

```math
v_t = \lfloor v_0 \times (\frac{l_t}{l_0} \times 2 - 1) \rceil
```

where $l_t$ is the food's remaining lifetime.

### Kills and Deaths

When a snake attempts to move into a space that is occupied, the snake will die, and the occupant will be credited with a kill (unless it was a suicide).
If two snakes both attempt to move into the same space and collide head on, they are both credited with a kill.
If a snake attempts to move out of the play grid, it dies and nobody is credited with a kill.
If a snake's length shrinks below 1 (by eating rotten food), it will die.

## Configuration

Configure your game through the `config.yaml` file:

```yaml
width: 32 # Board width
height: 32 # Board height
turns: 1500 # Maximum game turns
turnsPerSecond: 0 # 0 = run as fast as possible

respawn: 2 # Turns you miss while dead (0 = spawn next turn)

food:
  value: 5 # Initial value of food
  lifetime: 50 # How long food remains (0 = forever)
  count: 1 # Number of food items on the board at any point in time

players:
  - My Snake:
      type: custom
      cmd: mysnake.exe
      args:
        - --difficulty
        - hard
      timeout: 250 # Per-player move budget in milliseconds
      wait: false # If true, the game won't progress until a move is submitted by this agent

  - Easy Bot:
      type: builtin
      difficulty: easy
      color: red # You can set custom colors for your snake

   - Medium Bot:
      type: builtin
      difficulty: medium
      color: pacific blue # You can use Crayola color names

   - Hard Bot:
      type: builtin
      difficulty: hard
      color: fcd667 # Or you can use hex strings (with or without a '#')
```

## Agent Types

- **Custom**: External program that communicates via standard I/O
- **Builtin**: Pre-programmed AI with difficulty levels (easy/medium/hard)
- **Random**: Makes random moves
- **Keyboard**: Human-controlled via keyboard input

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

- A line containing the food count: `<apple_count>`
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

## FAQ

### What does a good snake look like?

That’s entirely up to you! There are countless strategies and ways to succeed. You might aim for maximum growth, stay small and focus on blocking others, or find some quirky niche that makes your snake unique.

A “good snake” doesn’t even have to win reliably. It could be one that does something fun, uses an interesting algorithm, be written in a language you’ve never tried before, or compiles down to the tiniest possible executable.

This game is more about the journey than the destination. If your idea of fun isn’t crushing every opponent, that’s perfectly fine too!

### How can I debug my snake?

It can be quite tricky to debug a program that you aren't launching directly, but you can probably attach the debugger after it has been launched by the engine (you'll have to figure this out yourself for your chosen language, sorry!).\
I recommend setting

```yaml
wait: true
```

for your snake in `config.yaml`. This will ensure that the game waits for you when you hit a breakpoint.
Make liberal use of logging to `stderr` to see your snake's thought process too!

### Can I cheat?

No, you cannot. The only rules are those built into the game itself. If you are able to accomplish something within those rules, I consider it to be a valid strategy. However, I do reserve the right to patch any unintended exploits.

### What language should I code my snake in?

Anything you want, as long as you can make it read and write over standard I/O.

### Where do I start with something like this?

Have a look at the examples to get an idea on how you can approach it.
You can also join us on [Discord](https://discord.gg/zapH4Sz7wH) to ask for help or just hang out!

## License

[MIT License](LICENSE)
