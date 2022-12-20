# Snakes!
 [![](https://dcbadge.vercel.app/api/server/zapH4Sz7wH)](https://discord.gg/zapH4Sz7wH)

"Snake" is a classic arcade game where the player controls a snake which gets longer every time it eats. If the snake collides with anything, including itself (which is more likely to happen as it gets longer), it will die and its length will be reset. This is a client for a Snake AI competition, a coding challenge in which participants must write a program that will play a multiplayer version of Snake against other snake AIs. The snake that holds the record for the longest length throughout the course of the game is the winner!

## Installation

Install Snakes! by cloning this repository and building with Cargo, or by downloading a [release](https://github.com/dylanHanger/snakes/releases)

```bash
  git clone https://github.com/dylanHanger/snakes
  cd snakes
  cargo build --release
```

## Run Locally

Simply install and run `snakes[.exe]`
```
USAGE:
    snakes[.exe] [OPTIONS]

OPTIONS:
    -c, --config <FILE>    The file to read game settings from [default: config.yaml]
    -h, --headless         Run in headless mode, without any graphical output
        --help             Print help information
    -V, --version          Print version information
```

## Gameplay

The goal of the game is to become the longest snake.
To do this you must eat fresh apples and avoid crashing.
But remember, this is a multiplayer game, and the other snakes are trying to do the same thing.

### Food

There is always one apple on the board at any time.
If one is eaten, another will immediately spawn at a random position.
Each turn, the apples decay; becoming rotten and eventually disappearing.

Initially, an apple is worth `value` (default: 5). As the apple decays, this will decrease linearly.
At the halfway point in its decay, the apple becomes rotten and eating it will shrink your snake.
After eating the apple, your snake will grow by one each turn for $v$ turns.

$$ v_t = \lfloor v_0 \times ({l_t \over l_0} \times 2 - 1) \rceil $$

where $l_i$ is the apple's remaining lifetime.

### Kills and Deaths

When a snake attempts to move into a space that is occupied, the snake will die, and the occupant will be credited with a kill (you will lose a kill if it was a suicide). If two snakes collide head on, they both get credited with a kill. If a snake attempts to move out of the play grid, it is killed, but nobody is credited with a kill. If a snake's length shrinks below 1 (by eating rotten food), it will die.

### Scoring

Snakes are ranked according to their stats in the following order:

 1. The maximum length the snake has achieved
 2. The number of kills they have (more is better)
 3. The number of deaths they have (less is better)
 4. The current length of the snake

## Configuration

You can configure the game settings via a YAML file. By default, this file is called `config.yaml`.

```yaml
---
width: <arena width>
height: <arena height>

seed: <seed for RNG>

replays:
  record: <whether to record replays or not>
  path: <the folder to save replays under>
  format: <format pattern for filenames>

turns: <max turns>
timeout: <allowed computation time>
wait: <whether to wait for all snakes to compute a move>

respawn: <number of turns a snake remains dead>

food:
  value: <growth value of a fresh apple>
  lifetime: <number of turns the apple lasts>

players:
  # A custom snake AI
  - <Snake name>:
      type: custom
      silent: <whether to suppress logging from this snake>
      executable: <executable to run>
      args:
        - <arg1>
        - <arg2>
        - ...
        - <argN>

  # A keyboard controlled snake
  - <Snake name>:
      type: keyboard
      keys:
        north: <key for up>
        west: <key for left>
        south: <key for down>
        east: <key for right>

  # A snake with a built in AI
  - <Snake name>:
      type: builtin
      difficulty: <easy | medium | hard>

  # A snake that moves randomly
  - <Snake name>:
      type: random
```

## Custom Agents
You can use any executable for your own custom AI agent. All communication between the game and the agent is performed over standard I/O.

### Game Initialization
When the game starts, the following information is sent once
```
<arena width> <arena height>
<food lifetime> <food value>
<number of players> <your id>
<number of turns> <timeout>
```
If `wait: true` is set in the `config.yaml`, then `timeout` will be `-1`.

### Update Loop
At the beginning of each turn, the following information is provided to every **living** snake.
```
<number of apples>
<lifetime0> <x0> <y0>
<lifetime1> <x1> <y1>
...
<lifetimeN> <xN> <yN>
<id0> <kills> <deaths> <length> <x1 y1 x2 y2 x3 y3 ...>
<id1> <kills> <deaths> <length> <x1 y1 x2 y2 x3 y3 ...>
...
<idN> <kills> <deaths> <length> <x1 y1 x2 y2 x3 y3 ...>
```
Dead snakes will have a length of 0 (and therefore will have no coordinates).
You then have some `timeout` (default: 250) milliseconds to compute your move. When it is ready, print it to standard out.

#### Possible Moves
| **Direction** | **Output**            | **Description**    |
| ------------- | --------------------- | ------------------ |
| North         | `0` \| `north` \| `n` | Turn **upwards**   |
| East          | `1` \| `east` \| `e`  | Turn **right**     |
| South         | `2` \| `south` \| `s` | Turn **downwards** |
| West          | `3` \| `west` \| `w`  | Turn **left**      |

Snakes cannot turn in the direction opposite to their current direction. For example, if you are moving North, an output of `2` (*South*) will be ignored.
If you do not output a move in time, your snake will simply move forwards.

#### Logging
If you wish to log to the console, print to standard error instead of standard out.

## Examples
Here is an example of a Python agent, `monty.py`
```python
import sys
import random

width, height = input().split()
value, lifetime = input().split()
num_players, my_id = input().split()
num_turns, timeout = input().split()

while True:
    try:
        num_apples = int(input())
        for _ in range(num_apples):
            # Read the food
            lifetime, apple_x, apple_y = input().split()

        num_snakes = int(input())
        for _ in range(num_snakes):
            # Read the snake
            (id, kills, deaths, length, *body) = input().split()

            # Do some processing
            print(f"Snake {id} is {length} long", file=sys.stderr)

        # Output a random move
        print(random.randint(0, 3))

    except(EOFError):
        # If the game closes, we need to exit gracefully
        exit()
```

And here is an example `example.yaml`
```yaml
---
width: 32
height: 32

turns: 1500
timeout: 25
wait: false
seed: the holy grail

replays:
  record: true
  path: replays/
  format: "{seed}-{time:%Y-%m-%dT%H-%M-%S}"

respawn: 10

food:
  value: 5
  lifetime: 50

players:
  - Monty:
      type: custom
      silent: false
      executable: python
      args:
        - monty.py

  - Human:
      type: keyboard
      keys:
        north: W
        west: A
        south: S
        east: D

  - Randy:
      type: random

  - Hard:
      type: builtin
      difficulty: hard
```
This can then be run with the command `./snakes[.exe] --config example.yaml`

## FAQ

#### How can I debug my snake?
If you are using Visual Studio Code, you can attach the debugger to your code after it has been launched by the game. Set
```yaml
wait: true
```
 in `config.yaml` to make sure the game waits for your snake if you hit any breakpoints.

#### Can I cheat?
Anything you can manage to do is a valid strategy, but I reserve the right to patch out any unintended exploits.

#### What language should I code my snake in?
Anything you want, as long as you can launch it as an executable that uses stdio.

#### Isn't a whole game engine (Bevy) a little overkill?
Yes. Yes it is.

## Roadmap

- [x] A UI scoreboard
- [x] Pausing
- [x] Single Stepping
- [ ] Replays (In progress)
- [ ] An actual winner
- [ ] Automated tournaments
- [ ] Web based viewer and leaderboard
