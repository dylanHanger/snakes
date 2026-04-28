# Debugging

Debugging a snake is trickier than a normal program because the engine launches your code, not you.
The sections below cover the tools the engine gives you for working around that.

## Pausing and single-stepping

You can pause the game by pressing <kbd>Space</kbd>, and step through a single turn with <kbd>Enter</kbd>.
This can be helpful when you are trying to work out what your snake _should_ be doing.

> [!NOTE]
> Pausing and single stepping are only available in windowed mode, and are not available in headless mode

## Debug logging

Anything you write to the standard error stream (`stderr`) will be displayed as a chat message by the game.
This allows you to write out your snake's thoughts, and allow you to inspect behaviour closely.
Set `silent: true` in the configuration for a snake to suppress its logs.

The chat will appear in the engine console, prefixed with your snake's name.
<pre>
[<span style="color: #C62D42">My Snake</span>] Hello world!
[<span style="color: #C62D42">My Snake</span>] I will move West
</pre>

It might be useful to spend some time creating helper methods for doing things like printing your snake's view of the world.

<pre>
[<span style="color: #C62D42">My Snake</span>]: Turn 76:
[<span style="color: #C62D42">My Snake</span>]:  <span style="color: #585858">•  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •</span>
[<span style="color: #C62D42">My Snake</span>]:  <span style="color: #585858">•  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •</span>
[<span style="color: #C62D42">My Snake</span>]:  <span style="color: #585858">•  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •</span>
[<span style="color: #C62D42">My Snake</span>]:  <span style="color: #585858">•  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •</span>
[<span style="color: #C62D42">My Snake</span>]:  <span style="color: #585858">•  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •</span>
[<span style="color: #C62D42">My Snake</span>]:  <span style="color: #585858">•  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •</span>
[<span style="color: #C62D42">My Snake</span>]:  <span style="color: #585858">•  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •</span>
[<span style="color: #C62D42">My Snake</span>]:  <span style="color: #585858">•  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •</span>
[<span style="color: #C62D42">My Snake</span>]:  <span style="color: #585858">•  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •</span>
[<span style="color: #C62D42">My Snake</span>]:  <span style="color: #585858">•  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •</span>
[<span style="color: #C62D42">My Snake</span>]:  <span style="color: #585858">•  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •</span>
[<span style="color: #C62D42">My Snake</span>]:  <span style="color: #585858">•  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •</span>
[<span style="color: #C62D42">My Snake</span>]:  <span style="color: #585858">•  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •</span>
[<span style="color: #C62D42">My Snake</span>]:  <span style="color: #585858">•  •  •  •  •  •  •  •  •  •  •  •  •  •</span>  <span style="color: #C62D42">@</span>  <span style="color: #585858">•  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •</span>
[<span style="color: #C62D42">My Snake</span>]:  <span style="color: #585858">•  •  •  •  •  •  •  •  •  •  •  •  •  •</span>  <span style="color: #C62D42">║</span>  <span style="color: #585858">•  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •</span>
[<span style="color: #C62D42">My Snake</span>]:  <span style="color: #585858">•  •  •  •  •  •  •  •  •  •  •  •  •  •</span>  <span style="color: #C62D42">║</span>  <span style="color: #585858">•  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •</span>
[<span style="color: #C62D42">My Snake</span>]:  <span style="color: #585858">•  •  •  •  •  •  •  •  •  •  •  •  •  •</span>  <span style="color: #C62D42">╚═══</span>  <span style="color: #585858">•  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •</span>
[<span style="color: #C62D42">My Snake</span>]:  <span style="color: #585858">•  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •</span>
[<span style="color: #C62D42">My Snake</span>]:  <span style="color: #585858">•  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •</span>
[<span style="color: #C62D42">My Snake</span>]:  <span style="color: #585858">•  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •</span>
[<span style="color: #C62D42">My Snake</span>]:  <span style="color: #585858">•  •</span>  <span style="color: #02A4D3">║</span>  <span style="color: #585858">•  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •</span>
[<span style="color: #C62D42">My Snake</span>]:  <span style="color: #585858">•  •</span>  <span style="color: #02A4D3">╚══╗</span>  <span style="color: #585858">•  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •</span>
[<span style="color: #C62D42">My Snake</span>]:  <span style="color: #585858">•  •  •</span>  <span style="color: #02A4D3">╚══╗</span>  <span style="color: #585858">•</span>  <span style="color: #5E8C31">═══════════════════════════@</span>  <span style="color: #585858">•  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •</span>
[<span style="color: #C62D42">My Snake</span>]:  <span style="color: #585858">•  •</span>  <span style="color: #02A4D3">╔═════╝</span>  <span style="color: #585858">•</span>  <span style="color: #E6BC5C">║</span>  <span style="color: #585858">•  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •</span>
[<span style="color: #C62D42">My Snake</span>]:  <span style="color: #585858">•  •</span>  <span style="color: #02A4D3">╚══╗</span>  <span style="color: #02A4D3">@══╗</span>  <span style="color: #E6BC5C">║</span>  <span style="color: #585858">•  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •</span>
[<span style="color: #C62D42">My Snake</span>]:  <span style="color: #585858">•  •  •</span>  <span style="color: #02A4D3">╚═════╝</span>  <span style="color: #E6BC5C">║</span>  <span style="color: #585858">•  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •</span>
[<span style="color: #C62D42">My Snake</span>]:  <span style="color: #585858">•  •  •  •  •  •</span>  <span style="color: #E6BC5C">║</span>  <span style="color: #585858">•  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •</span>
[<span style="color: #C62D42">My Snake</span>]:  <span style="color: #585858">•  •  •  •  •  •</span>  <span style="color: #E6BC5C">╚══════════════@ ─────────────────────────── ●</span>  <span style="color: #585858">•  •  •  •  •  •  •  •  •  •</span>
[<span style="color: #C62D42">My Snake</span>]:  <span style="color: #585858">•  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •</span>
[<span style="color: #C62D42">My Snake</span>]:  <span style="color: #585858">•  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •</span>
[<span style="color: #C62D42">My Snake</span>]:  <span style="color: #585858">•  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •</span>
</pre>

You can print [ANSI color codes](https://en.wikipedia.org/wiki/ANSI_escape_code#Colors) to enrich your logs.

## Attaching a debugger

This is by far the most powerful method to debug your snake, but it can be difficult to set up, and depends on your language and development environment.
The key is to manually spawn and attach a debugger for your language (such as `debugpy`, `lldb`, `delve`, etc).
The way you do this depends greatly on how familiar you are with your tools.

You should also set `wait: true` for your snake, to ensure that the game waits for you when you hit a breakpoint.

Look at the [examples/](/examples/) to see this in action for various languages and VS Code.

## Seeded runs

Set a specific seed when testing, to ensure that you get the same spawn locations and you can see if you really did fix a problem, or if it just never came up again.
But remember, just because you do well on one seed, doesn't mean you will do well on every seed, and vice versa.
Make sure you try at least a few different seeds to ensure you aren't getting (un)lucky.
Keep in mind that the seed does not affect any internal randomness that you have in your own code, so you should seed that too (it probably doesn't matter if you have the same every time)

> [!CAUTION]
> In the current version (v0.9.1) there are some bugs that affect the determinism of the built in agents, and seeds may not guarantee a perfectly repeatable game.
