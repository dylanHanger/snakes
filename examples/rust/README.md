# Rust Example

This is a very basic example Rust agent. The code is deliberately minimal; its purpose is to demonstrate how to read the game state and output a move, not to provide a competitive strategy.

## Communication

The example includes a set of helper macros for parsing input from _Snakes!_.

- `read!` and `readln!` let you specify the types you expect, and return the parsed values.

```rust
let (game_width, game_height) = read!(usize, usize);
```

## Debugging

For development, the example includes some basic self-debugging support.

- If you run your snake with `--attach-debugger`, it will attempt to launch VS Code and attach the [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) extension automatically, then pause for a few seconds.
- If the auto-attach doesn’t work, you can always attach manually to the process ID it prints.

### VS Code

The included `.vscode/launch.json` shows how to configure your IDE to run the _game host_ (`snakes`) instead of your snake directly. Remember: you don’t launch your agent, you launch _Snakes!_ and it will handle running the agent.

## Strategy

This example agent doesn't try to be smart. It simply cycles through a fixed sequence of moves:

```rust
const DIRECTIONS: [&str; 6] = ["north", "east", "east", "south", "west", "west"];
```

This is scaffolding for you to build on. For anything beyond a toy example, you’ll want to:

- Organize your code with proper structs and functions.
- Implement decision-making logic.
- Experiment with pathfinding and strategies.
