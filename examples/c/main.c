#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifdef _WIN32
#include <windows.h>
#else
#include <unistd.h>
#endif

void attach_debugger(int argc, char **argv) {
#ifdef _WIN32
  DWORD pid = GetCurrentProcessId();
#else
  pid_t pid = getpid();
#endif

  char url[256];
  snprintf(url, sizeof(url),
           "vscode://vadimcn.vscode-lldb/launch/"
           "config?{'request':'attach','pid':%ld}",
           (long)pid);

#ifdef _WIN32
  ShellExecuteA(NULL, "open", url, NULL, NULL, SW_SHOWNORMAL);
#else
  const char *cmd = "xdg-open";
#ifdef __APPLE__
  cmd = "open";
#endif
  if (fork() == 0) {
    execlp(cmd, cmd, url, (char *)NULL);
    _exit(1);
  }
#endif

  fprintf(stderr, "Waiting for VS Code to attach to PID %ld...\n", (long)pid);
  fflush(stderr);

#ifdef _WIN32
  Sleep(3000);
#else
  sleep(3);
#endif
}

const char *DIRECTIONS[] = {"north", "east", "east", "south", "west", "west"};
const int NUM_DIRECTIONS = 6;

int main(int argc, char **argv) {
  for (int i = 1; i < argc; i++) {
    if (strcmp(argv[i], "--attach-debugger") == 0) {
      attach_debugger(argc, argv);
      break;
    }
  }

  int game_width, game_height;
  int food_lifetime_raw, food_value;
  int num_snakes, my_id;
  int max_turns, timeout_raw;

  // Game initialization
  if (scanf("%d %d", &game_width, &game_height) != 2)
    return 1;
  if (scanf("%d %d", &food_lifetime_raw, &food_value) != 2)
    return 1;
  if (scanf("%d %d", &num_snakes, &my_id) != 2)
    return 1;
  if (scanf("%d %d", &max_turns, &timeout_raw) != 2)
    return 1;

  int food_lifetime = food_lifetime_raw > 0 ? food_lifetime_raw : -1;
  int timeout = timeout_raw > 0 ? timeout_raw : -1;

  fprintf(stderr, "I am #%d of %d.\n", my_id, num_snakes);
  if (food_lifetime > 0) {
    fprintf(stderr, "Food is worth %d and lasts for %d turns.\n", food_value,
            food_lifetime);
  } else {
    fprintf(stderr, "Food is worth %d and lasts forever.\n", food_value);
  }
  fprintf(stderr, "The board size is %dx%d.\n", game_width, game_height);
  if (timeout > 0) {
    fprintf(stderr, "There are %d turns, and I have %dms per move.\n",
            max_turns, timeout);
  } else {
    fprintf(stderr, "There are %d turns, with infinite time per move.\n",
            max_turns);
  }

  // Update loop
  int current_turn = 0;
  while (1) {
    current_turn++;
    fprintf(stderr, "Turn %d\n", current_turn);

    // Food
    int num_food;
    if (scanf("%d", &num_food) != 1)
      break;
    for (int i = 0; i < num_food; i++) {
      int lifetime, x, y;
      if (scanf("%d %d %d", &lifetime, &x, &y) != 3)
        return 1;
      if (food_lifetime > 0) {
        int value = (int)((double)food_value *
                          (((double)lifetime / food_lifetime * 2) - 1));
        fprintf(stderr,
                "Food at (%d,%d) has %d turns remaining. It is worth %d\n", x,
                y, lifetime, value);
      } else {
        fprintf(stderr, "Food at (%d,%d) is worth %d and does not rot.\n", x, y,
                food_value);
      }
    }

    // Snakes
    for (int s = 0; s < num_snakes; s++) {
      int snake_id, kills, deaths, length;
      if (scanf("%d %d %d %d", &snake_id, &kills, &deaths, &length) != 4)
        return 1;

      for (int i = 0; i < length; i++) {
        int x, y;
        if (scanf("%d %d", &x, &y) != 2)
          return 1;
        if (i == 0) {
          fprintf(stderr, "Snake #%d starts at (%d,%d), length %d, K/D %d/%d\n",
                  snake_id, x, y, length, kills, deaths);
        }
      }
    }

    // Pick a move
    const char *dir = DIRECTIONS[current_turn % NUM_DIRECTIONS];
    fprintf(stderr, "I am going to move %s\n", dir);
    printf("%s\n", dir);
    fflush(stdout);

    fprintf(stderr, "================================\n");
  }

  return 0;
}
