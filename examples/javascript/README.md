# JavaScript Example

This is a very basic example JavaScript agent, intended to be run with [Node.js](https://nodejs.org/). The code is deliberately minimal; its purpose is to demonstrate how to read the game state and output a move, not to provide a competitive strategy.

## Communication

The agent reads stdin line-by-line via `readline` and a small `nextInts()` helper that splits each line into integers. It also provides a `say` method to make it easier to print to `stderr`.

## Debugging

For development, the example includes some basic self-debugging support.

- If you run your snake with `--attach-debugger`, it will open Node's built-in inspector on port `9229` and wait for a connection.

### VS Code

The included `.vscode/launch.json` shows how to configure your IDE to run the _game host_ (`snakes`) instead of your snake directly. Remember: you don't launch your agent, you launch _Snakes!_ and it will handle running the agent.

The configuration will launch snakes via the `Launch Snakes!` task, then attach the Node debugger on port `9229`.

## Strategy

This example agent doesn't try to be smart. It simply cycles through a fixed sequence of moves:

```javascript
const DIRECTIONS = ['north', 'east', 'east', 'south', 'west', 'west'];
```

This is scaffolding for you to build on. For anything beyond a toy example, you'll want to:

- Organize your code into proper modules and classes.
- Implement decision-making logic.
- Experiment with pathfinding and strategies.
