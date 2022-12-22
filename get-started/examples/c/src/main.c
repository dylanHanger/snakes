#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <time.h>

int main(int argc, char **argv)
{
    int width, height;
    int food_lifetime, food_value;
    int num_players, my_id;
    int num_turns, timeout;

    fprintf(stderr, "Hello, world!\n");
    scanf("%d %d", &width, &height);
    fprintf(stderr, "width=%d, height=%d\n", width, height);
    scanf("%d %d", &food_lifetime, &food_value);
    fprintf(stderr, "lifetime=%d, value=%d\n", food_lifetime, food_value);
    scanf("%d %d", &num_players, &my_id);
    fprintf(stderr, "players=%d, my_id=%d\n", num_players, my_id);
    scanf("%d %d", &num_turns, &timeout);
    fprintf(stderr, "turns=%d, timeout=%d\n", num_turns, timeout);
    fflush(stderr);

    srand(time(NULL));
    while (true)
    {
        // Read the current game state
        // WARNING: This is just an example. You should use better data structures, such as your own structs and types.
        int num_apples;
        scanf("%d", &num_apples);
        int **apples = malloc(num_apples * sizeof(int *));
        for (int i = 0; i < num_apples; i++)
        {
            int lifetime, x, y;
            scanf("%d %d %d", &lifetime, &x, &y);
            apples[i] = malloc(3 * sizeof(int));
            apples[i][0] = lifetime;
            apples[i][1] = x;
            apples[i][2] = y;
        }

        int **snakes = malloc(num_players * sizeof(int *));
        for (int i = 0; i < num_players; i++)
        {
            int id, kills, deaths, len;
            scanf("%d %d %d %d", &id, &kills, &deaths, &len);
            snakes[i] = malloc((2 + 2 * len) * sizeof(int));
            snakes[i][0] = id;
            snakes[i][1] = len;
            for (int j = 0; j < len; j++)
            {
                scanf("%d %d", &snakes[i][2 + 2 * j], &snakes[i][2 + 2 * j + 1]);
            }
        }

        // Compute an action
        int action = rand() % 4;

        // You can output debug information to STDERR
        fprintf(stderr, "I am snake %d and I chose action %d\n", my_id, action);
        fflush(stderr);

        // Output the action
        printf("%d\n", action);
        fflush(stdout);

        // Free the memory allocated for the apples array
        for (int i = 0; i < num_apples; i++)
        {
            free(apples[i]);
        }
        free(apples);

        // Free the memory allocated for the snakes array
        for (int i = 0; i < num_players; i++)
        {
            free(snakes[i]);
        }
        free(snakes);
    }

    return 0;
}
