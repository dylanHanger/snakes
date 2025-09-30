# Examples

This folder contains a collection of _very basic_ example snake agents.  
They are not designed to be competitive, nor do they contain good programming style.  
Their only purpose is to show you how to interact with the Snakes! engine and to give you a starting point for writing your own agent.

## Debugging

### Tips

- Set `wait: true` for your snake in your config.  
  This prevents the engine from timing out your snake while it’s paused at a breakpoint.
- Be loud! Send debug logs to `stderr` so you can trace what your snake is thinking.
- Attach a debugger. The exact method depends on your language, environment, and IDE. You will have to play around a bit to find what works best.

### Visual Studio Code

Each example includes a Visual Studio Code setup that demonstrates how to setup launch configurations.  
These configurations assume that the `snakes` binary is on your `PATH` (so you can run `snakes` directly in a terminal).
