import random
import sys
import debugpy  # pip install debugpy
import os

if __name__ == "__main__":
    if os.getenv("DEBUG", False):
        # This block of code will only run when the DEBUG environment variable is set to True
        # https://blog.oyam.dev/vscode-python-debugger/ explains how to attach this to VS Code
        # This isn't a super nice way to do this, but unfortunately it's the only way I've found
        debugpy.listen(5678)
        print("Waiting for debugger attach...")
        debugpy.wait_for_client()

    width, height = input().split()
    food_value, food_lifetime = input().split()
    num_players, my_id = input().split()
    num_turns, timeout = input().split()

    while True:
        # Read the current game state
        # WARNING: This is just an example. You should use better data structures.
        num_apples = int(input())
        for i in range(num_apples):
            lifetime, (x, y) = input().split()

        for i in range(num_players):
            snake_id, kills, deaths, length, *body = input().split()

        # Compute an action
        action = random.randint(0, 3)

        # You can output debug information to STDERR
        print(f"I am snake {my_id} and I chose action {action}", file=sys.stderr)

        # Output the action
        print(action)
