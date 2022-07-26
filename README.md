﻿# Snakes!

"Snake" is a classic arcade game where the player controls a snake which gets longer every time it eats. If the snake collides with anything, including itself (which is more likely to happen as it gets longer), it will die and its length will be reset. This is a client for a Snake AI competition, a coding challenge in which participants must write a program that will play a multiplayer version of Snake against other snake AIs. The snake that holds the record for the longest length throughout the course of the game is the winner!

## Installation

Install Snakes! by cloning this repository and building with Cargo, or by downloading a release

```bash
  git clone https://github.com/dylanHanger/snakes
  cd snakes
  cargo build --release
```
    
## Run Locally

Simply run `snakes[.exe]`, config is read from a file called `config.yaml`

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

$$ v_t = \lfloor V \times ({l_t \over l_0} \times 2 - 1) \rceil $$ where $l_i$ is the apple's remaining lifetime.

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

turns: <max turns>
timeout: <allowed computation time>
wait: <whether to wait for all snakes to compute a move>

respawn: <number of turns a snake remains dead>

food:
  value: <growth value of a fresh apple>
  lifetime: <number of turns the apple lasts>

snakes:
  # A custom snake AI
  - <Snake name>:
      type: custom
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
<your snake id>
```

### Update Loop
At the beginning of each turn, the following information is provided to every **living** snake
```
<number of apples>
<apple0_x> <apple0_y>
<apple1_x> <apple1_y>
...
<appleN_x> <appleN_y>
<number of living snakes>
<id0> <length> <x1 y1 x2 y2 x3 y3 ...>
<id1> <length> <x1 y1 x2 y2 x3 y3 ...>
...
<idN> <length> <x1 y1 x2 y2 x3 y3 ...>
```
If a snake is dead, it will not be included in the update.
You then have some `timeout` (default: 250) milliseconds to compute your move. When it is ready, print it to standard out.

#### Possible Moves
| **Direction** | **Output** | **Description** |
|--|--| -- |
| North | `0` \| `north` \| `n` | Turn **upwards** |
| East | `1` \| `east` \| `e` | Turn **right** |
| South | `2` \| `south` \| `s` | Turn **downwards** |
| West | `3` \| `west` \| `w` | Turn **left** |

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
my_snake = input()

while True:
    try:
        num_apples = int(input())
        for _ in range(num_apples):
            # Read the food
            apple_x, apple_y = input().split()

        num_snakes = int(input())
        for _ in range(num_snakes):
            # Read the snake
            (id, length, *body) = input().split()

            # Do some processing
            print(f"Snake {id} is {length} long", file=sys.stderr)

        # Output a random move
        print(random.randint(1, 4))

    except(EOFError):
        # If the game closes, we need to exit gracefully
        exit()
```

And here is an example `config.yaml`
```yaml
---
width: 32
height: 32

turns: 1500
timeout: 25
wait: false

respawn: 10

food:
  value: 5
  lifetime: 50

snakes:
  - Monty:
      type: custom
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
This can then be run with the command `./snakes[.exe]`

## FAQ

#### How can I debug my snake?
If you are using Visual Studio Code, you can attach the debugger to your code after it has been launched by the game. Set `wait: true`  in `config.yaml` to make sure the game waits for your snake if you hit any breakpoints.

#### Can I cheat?
Anything you can manage to do is a valid strategy, but I reserve the right to patch out any unintended exploits.

#### What language should I code my snake in?
Anything you want, as long as you can launch it as an executable that uses stdio.

## Roadmap

- [ ] A UI scoreboard
- [ ] An actual winner
- [ ] Replays, pausing, single stepping
- [ ] Automated tournaments
- [ ] Web based viewer and leaderboard