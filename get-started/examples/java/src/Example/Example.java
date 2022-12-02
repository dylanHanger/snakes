package Example;

import java.io.*;
import java.util.Random;

class Example {
    public static void main(String[] args) {
        try (BufferedReader br = new BufferedReader(new InputStreamReader(System.in))) {
            String initString = br.readLine();
            String[] initStringArray = initString.split(" ");

            int width = Integer.parseInt(initStringArray[0]);
            int height = Integer.parseInt(initStringArray[1]);

            int my_id = Integer.parseInt(br.readLine());

            while (true) {
                // Read the current game state
                // WARNING: This is just an example. You should use classes and objects to represent the game state
                int num_apples = Integer.parseInt(br.readLine());
                for (int i = 0; i < num_apples; i++) {
                    String appleString = br.readLine();
                    String[] appleStringArray = appleString.split(" ");
                    int apple_x = Integer.parseInt(appleStringArray[0]);
                    int apple_y = Integer.parseInt(appleStringArray[1]);
                }

                int num_snakes = Integer.parseInt(br.readLine());
                for (int i = 0; i < num_snakes; i++) {
                    String snakeString = br.readLine();
                    String[] snakeStringArray = snakeString.split(" ");
                    int snake_id = Integer.parseInt(snakeStringArray[0]);
                    int[] body = new int[snakeStringArray.length - 1];
                    for (int j = 1; j < snakeStringArray.length; j++) {
                        body[j - 1] = Integer.parseInt(snakeStringArray[j]);
                    }
                }

                // Compute an action
                Random r = new Random();
                int action = r.nextInt(4);

                // You can output debug information to STDERR
                System.err.println(String.format("I am snake %d and I chose action %d", my_id, action));

                // Output the action
                System.out.println(action);
            }
        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}
