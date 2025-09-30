using System.Diagnostics;

class Program
{
    static void Main(string[] args)
    {
#if DEBUG
        if (args.Contains("--attach-debugger"))
        {
            Console.Error.WriteLine($"Waiting for debugger to attach to PID {Environment.ProcessId}");
            while (!Debugger.IsAttached)
            {
                Thread.Sleep(100);
            }
        }
#endif

        // Initialization
        var parts = Console.ReadLine()!.Split();
        int gameWidth = int.Parse(parts[0]);
        int gameHeight = int.Parse(parts[1]);

        parts = Console.ReadLine()!.Split();
        int foodLifetimeRaw = int.Parse(parts[0]);
        float foodValue = float.Parse(parts[1]);

        parts = Console.ReadLine()!.Split();
        int numSnakes = int.Parse(parts[0]);
        uint myId = uint.Parse(parts[1]);

        parts = Console.ReadLine()!.Split();
        int maxTurns = int.Parse(parts[0]);
        int timeoutRaw = int.Parse(parts[1]);

        float? foodLifetime = foodLifetimeRaw > 0 ? foodLifetimeRaw : null;
        int? timeout = timeoutRaw > 0 ? timeoutRaw : null;

        Console.Error.WriteLine($"I am #{myId} of {numSnakes}.");
        Console.Error.WriteLine($"Food is worth {foodValue} and lasts {(foodLifetime.HasValue ? $"for {foodLifetime} turns" : "forever")}.");
        Console.Error.WriteLine($"The board size is {gameWidth}x{gameHeight}.");
        Console.Error.WriteLine($"There are {maxTurns} turns, and I have {(timeout.HasValue ? $"{timeout}ms" : "infinite time")} to make each move.");

        // Update loop
        int currentTurn = 0;
        string[] directions = ["north", "east", "east", "south", "west", "west"];

        while (true)
        {
            currentTurn++;
            Console.Error.WriteLine($"Turn {currentTurn}");

            int numFood = int.Parse(Console.ReadLine()!);
            for (int i = 0; i < numFood; i++)
            {
                parts = Console.ReadLine()!.Split();
                float lifetime = float.Parse(parts[0]);
                uint x = uint.Parse(parts[1]);
                uint y = uint.Parse(parts[2]);

                if (foodLifetime.HasValue)
                {
                    float value = (float)Math.Round(foodValue * (lifetime / foodLifetime.Value * 2 - 1));
                    Console.Error.WriteLine($"The food at ({x},{y}) has {lifetime} turns remaining. It is worth {value}");
                }
                else
                {
                    Console.Error.WriteLine($"The food at ({x},{y}) is worth {foodValue} and will not rot.");
                }
            }

            for (int s = 0; s < numSnakes; s++)
            {
                parts = Console.ReadLine()!.Split();
                uint id = uint.Parse(parts[0]);
                int kills = int.Parse(parts[1]);
                uint deaths = uint.Parse(parts[2]);

                int length = int.Parse(parts[3]);
                for (int i = 0; i < length; i++)
                {
                    var x = int.Parse(parts[2 * i + 4]);
                    var y = int.Parse(parts[2 * i + 5]);
                    if (i == 0)
                    {
                        Console.Error.WriteLine($"Snake #{id} starts at ({x},{y}), is {length} long, and has a K/D of {kills}/{deaths}");
                    }
                }
            }

            string direction = directions[currentTurn % directions.Length];
            Console.Error.WriteLine($"I am going to move {direction}");
            Console.WriteLine(direction);
            Console.Error.WriteLine("================================");
        }
    }
}
