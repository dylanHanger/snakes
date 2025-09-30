import sys
import subprocess

DIRECTIONS = ["north", "east", "east", "south", "west", "west"]

def say(msg):
    """Print a debug line to stderr so it doesn't interfere with game output."""
    print(msg, file=sys.stderr, flush=True)

def main():
    # Game initialization
    game_width, game_height = [int(x) for x in input().split(' ')]
    food_lifetime_raw, food_value = [int(x) for x in input().split(' ')]
    num_snakes, my_id = [int(x) for x in input().split(' ')]
    max_turns, timeout_raw = [int(x) for x in input().split(' ')]

    food_lifetime = food_lifetime_raw if food_lifetime_raw > 0 else None
    timeout = timeout_raw if timeout_raw > 0 else None

    say(f"I am #{my_id} of {num_snakes}.")
    if food_lifetime:
        say(f"Food is worth {food_value} and lasts for {food_lifetime} turns.")
    else:
        say(f"Food is worth {food_value} and lasts forever.")
    say(f"The board size is {game_width}x{game_height}.")
    if timeout:
        say(f"There are {max_turns} turns, and I have {timeout}ms per move.")
    else:
        say(f"There are {max_turns} turns, with infinite time per move.")

    # Update loop
    current_turn = 0
    while True:
        current_turn += 1
        say(f"Turn {current_turn}")

        # Food
        num_food = int(input())
        for _ in range(num_food):
            lifetime, x, y = (int(x) for x in input().split(' '))
            if food_lifetime:
                value = round(food_value * ((lifetime / food_lifetime * 2) - 1))
                say(f"Food at ({x},{y}) has {lifetime} turns remaining. It is worth {value}")
            else:
                say(f"Food at ({x},{y}) is worth {food_value} and does not rot.")

        # Snakes
        for _ in range(num_snakes):
            snake_id, kills, deaths, length, *coords = [int(x) for x in input().split(' ')]
            for i in range(length):
                x, y = coords[i], coords[i+1]
                if i == 0:
                    say(f"Snake #{snake_id} starts at ({x},{y}), "
                          f"length {length}, K/D {kills}/{deaths}")

        # Pick a move
        direction = DIRECTIONS[current_turn % len(DIRECTIONS)]
        say(f"I am going to move {direction}")
        print(direction, flush=True)
        say("================================")

def attach_debugger():
    try:
        import debugpy
    except ImportError:
        say("debugpy is not installed. Run `pip install debugpy`")
        return

    debugpy.listen(("0.0.0.0", 5678))
    say("Waiting for debugger to attach on port 5678")
    debugpy.wait_for_client()
    say("Debugger attached")

if __name__ == "__main__":
    if "--attach-debugger" in sys.argv:
        attach_debugger()
    main()
