# Python Example

This is a very basic example Python agent. The code is deliberately minimal; its purpose is to demonstrate how to read the game state and output a move, not to provide a competitive strategy.

## Communication

This code just uses `input().split(' ')` to read an entire line of numbers at once.
It also provides `say` method to make it easier to print to stderr.

## Debugging

For development, the example includes some basic self-debugging support.

- If you run your snake with `--attach-debugger`, it will start a debugpy session and wait for a connection on port 5678.

### VS Code

The included `.vscode/launch.json` shows how to configure your IDE to run the _game host_ (`snakes`) instead of your snake directly. Remember: you don’t launch your agent, you launch _Snakes!_ and it will handle running the agent.
The configuration will launch snakes, and then attempt to attach to debugpy on port 5678. You must have debugpy installed for this to work.

## Strategy

This example agent doesn't try to be smart. It simply cycles through a fixed sequence of moves:

```python
DIRECTIONS = ["north", "east", "east", "south", "west", "west"]
```

This is scaffolding for you to build on. For anything beyond a toy example, you’ll want to:

- Organize your code with proper classes and functions.
- Implement decision-making logic.
- Experiment with pathfinding and strategies.
