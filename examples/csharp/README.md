# C# Example

This is a very basic example C# agent. The code is deliberately minimal; its purpose is to demonstrate how to read the game state and output a move, not to provide a competitive strategy.

## Debugging

This example does not contain any VS Code launch configurations, however the snake accepts an `--attach-debugger` flag, that when provided to a default build will wait for a debugger to be manually attached.

## Strategy

This example agent doesn't try to be smart. It simply cycles through a fixed sequence of moves:

```rust
const DIRECTIONS: [&str; 6] = ["north", "east", "east", "south", "west", "west"];
```

This is scaffolding for you to build on. For anything beyond a toy example, you�ll want to:

- Organize your code with proper structs and functions.
- Implement decision-making logic.
- Experiment with pathfinding and strategies.
