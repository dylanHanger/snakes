# C# Example

This is a very basic example C# agent. The code is deliberately minimal; its purpose is to demonstrate how to read the game state and output a move, not to provide a competitive strategy.

## Debugging

This example is set up for debugging in Visual Studio (not VS Code). The snake accepts an `--attach-debugger` flag that when provided to a `DEBUG` build, will wait for a debugger to be manually attached.

In Visual Studio you can attach a debugger by going to `Debug > Attach To Process` (<kbd>Ctrl+Alt+P</kbd>) and selecting `MyCSharpSnake.exe`.

## Strategy

This example agent doesn't try to be smart. It simply cycles through a fixed sequence of moves:

```csharp
string[] directions = ["north", "east", "east", "south", "west", "west"];
```

This is scaffolding for you to build on. For anything beyond a toy example, you'll want to:

- Organize your code with proper structs and functions.
- Implement decision-making logic.
- Experiment with pathfinding and strategies.
