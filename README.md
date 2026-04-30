# Snakes!

"Snake" is a classic arcade game where the player controls a snake which gets longer every time it eats. If the snake collides with anything, including itself (which is more likely to happen as it gets longer!), it will die and its length will be reset.
"Snakes!" is an AI competition, a coding challenge in which participants must write a program that will play a multiplayer version of Snake against other snake AIs. The snake that holds the record for the longest length throughout the course of the game is the winner!

## Quick Start

Grab the [latest release](https://github.com/dylanHanger/snakes/releases) for your platform.

**Linux / macOS:**
```bash
tar -xzf snakes-*.tar.gz
cd snakes-*
./snakes --config config.yaml
```

**Windows:** Extract the zip, open a terminal in the extracted folder, and run `snakes.exe --config config.yaml`.

If you would prefer to build from source:

```bash
git clone https://github.com/dylanHanger/snakes
cd snakes
go run ./cmd/snakes --config config/config.yaml
```

## Basic rules

The goal of the game is to grow longer than any other snake.
You do so by eating food which spawns randomly on the board.
The fresher the food, the more you will grow.
Old food becomes rotten and will shrink you instead.
If you crash into anything, or if you shrink too much, you die, and will respawn a few turns later and have to start growing again.

## Your first snake

Creating a snake is simple: it's just a program that reads the game state from standard input and prints a move (`n`, `e`, `s`, `w`) to standard output each turn.
Here is a simple Python example that makes random moves:

```python
import random

# Game initialization
# This game doesn't use most of this, but you will
game_width, game_height = [int(x) for x in input().split()]
food_lifetime, food_value = [int(x) for x in input().split()]
num_snakes, my_id = [int(x) for x in input().split()]
max_turns, respawn, timeout = [int(x) for x in input().split()]

# Update loop, this runs every turn
while True:
    # Food
    num_food = int(input())
    for _ in range(num_food):
        input()  # we just discard the food info

    # Snakes
    for _ in range(num_snakes):
        input()  # we just discard the snake info

    # Pick a move
    direction = random.choice(['n','e','s','w'])
    print(direction, flush=True)
```

Save that in a file called `my_snake.py`, then add it to your `config.yaml` under `players`:
```yaml
- My Snake:
    type: custom
    cmd: python
    args:
      - my_snake.py
```

See the [examples/](examples/) directory for simple examples of snakes in a variety of languages.

## Learn more

| If you want to... | Read |
|---|---|
| Learn the rules and communication in more detail | [docs/reference.md](docs/reference.md) |
| Learn about the configuration options | [docs/configuration.md](docs/configuration.md) |
| Debug your snake when it's not behaving | [docs/debugging.md](docs/debugging.md) |
| See more example agents | [examples/](examples/) |

## FAQ

### What language should I write my snake in?

Anything you want, as long as you can read and write standard I/O.
For a complete beginner, Python is very easy to pick up.

### How do I debug my snake?

Anything you print to `stderr` will be written as a chat message.
See [docs/debugging.md](docs/debugging.md) for more information.

### Can I cheat?

If you can do it, it's not cheating.
However, I do reserve the right to patch out any unintended exploits.

### What does a _good_ snake look like?

That's entirely up to you.
You can make a snake that wins every game, write the fewest lines of code possible, write one in a new language you have never used before, or any other goal you can think of.
It's about the journey, not the destination.

## License

[Apache License](LICENSE)
