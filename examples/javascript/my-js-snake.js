const readline = require('readline');
const inspector = require('inspector');

const DIRECTIONS = ['north', 'east', 'east', 'south', 'west', 'west'];

const say = (msg) => process.stderr.write(msg + '\n');

const createLineReader = () => {
  const rl = readline.createInterface({ input: process.stdin });
  const iter = rl[Symbol.asyncIterator]();

  const nextLine = async () => {
    const { value, done } = await iter.next();
    if (done) process.exit(0);
    return value;
  };

  return {
    nextLine,
    nextInts: async () => (await nextLine()).split(' ').map(Number),
  };
};

const formatFoodValue = (lifetime, maxLifetime, baseValue) =>
  Math.round(baseValue * ((lifetime / maxLifetime) * 2 - 1));

async function main() {
  const { nextInts } = createLineReader();

  // Game initialization
  const [gameWidth, gameHeight] = await nextInts();
  const [foodLifetimeRaw, foodValue] = await nextInts();
  const [numSnakes, myId] = await nextInts();
  const [maxTurns, timeoutRaw] = await nextInts();

  const foodLifetime = foodLifetimeRaw || null;
  const timeout = timeoutRaw || null;

  say(`I am #${myId} of ${numSnakes}.`);
  say(
    `Food is worth ${foodValue} and lasts ${
      foodLifetime ? `${foodLifetime} turns` : 'forever'
    }.`
  );
  say(`The board size is ${gameWidth}x${gameHeight}.`);
  say(
    `There are ${maxTurns} turns, ${
      timeout ? `${timeout}ms per move` : 'with infinite time per move'
    }.`
  );

  // Update loop
  for (let currentTurn = 1; ; currentTurn++) {
    say(`Turn ${currentTurn}`);

    // Food
    const [numFood] = await nextInts();

    for (let i = 0; i < numFood; i++) {
      const [lifetime, x, y] = await nextInts();

      if (foodLifetime) {
        const value = formatFoodValue(lifetime, foodLifetime, foodValue);
        say(
          `Food at (${x},${y}) has ${lifetime} turns remaining. It is worth ${value}`
        );
      } else {
        say(`Food at (${x},${y}) is worth ${foodValue} and does not rot.`);
      }
    }

    // Snakes
    for (let s = 0; s < numSnakes; s++) {
      const [snakeId, kills, deaths, length, ...coords] = await nextInts();

      if (!length) continue;

      const [x, y] = coords;
      say(
        `Snake #${snakeId} starts at (${x},${y}), length ${length}, K/D ${kills}/${deaths}`
      );
    }

    // Pick a move
    const direction = DIRECTIONS[currentTurn % DIRECTIONS.length];
    say(`I am going to move ${direction}`);

    process.stdout.write(direction + '\n');
    say('================================');
  }
}

if (process.argv.includes('--attach-debugger')) {
  inspector.open(9229, '127.0.0.1', true);
}

main();