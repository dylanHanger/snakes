# C Example

This is a very basic example C agent. The code is deliberately minimal; its purpose is to demonstrate how to read the game state and output a move, not to provide a competitive strategy.

## Communication

Read from `stdin` with `scanf`, and write logs with `fprintf(stder, ...)`.

## Building

This example uses `xmake` for cross-platform building.
You can build your snake with `xmake build`, which will put the executable in `build/{OS}/{ARCH}/{MODE}`, where `{MODE}` is either `build` or `release`.

## Debugging

For development, the example includes some basic self-debugging support.

- If you run your snake with `--attach-debugger`, it will attempt to launch VS Code and attach the [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) extension automatically, then pause for a few seconds.
- If the auto-attach doesn’t work, you can always attach manually to the process ID it prints.

**Note:** LLDB might not work correctly when using `gcc` instead of `clang`.

### VS Code

The included `.vscode/launch.json` shows how to configure your IDE to run the _game host_ (`snakes`) instead of your snake directly. Remember: you don’t launch your agent, you launch _Snakes!_ and it will handle running the agent.

## Strategy

This example agent doesn't try to be smart. It simply cycles through a fixed sequence of moves:

```c
const char *DIRECTIONS[] = {"north", "east", "east", "south", "west", "west"};
```

This is scaffolding for you to build on. For anything beyond a toy example, you’ll want to:

- Organize your code with proper structs and functions.
- Implement decision-making logic.
- Experiment with pathfinding and strategies.
