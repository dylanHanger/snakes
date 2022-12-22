pub mod macros;
use rand::Rng;

fn main() {
    let (width, height) = read!(usize, usize);
    let (food_lifetime, food_value) = read!(usize, i32);
    let (num_players, my_id) = read!(usize, usize);
    let (num_turns, timeout) = read!(u32, i64); // Timeout could be -1

    loop {
        // Read the current game state
        // WARNING: This is just an example. You should use better data structures, such as your own structs and types.
        let num_apples = read!(usize);
        let apples = (0..num_apples)
            .map(|_| read!(usize, i32, i32))
            .collect::<Vec<_>>();

        let snakes: Vec<_> = (0..num_players)
            .map(|_| {
                let id = read!(u32);
                let (kills, deaths, len) = read!(i32, u32, usize);
                let body = if len > 0 {
                    (0..len).map(|_| read!(usize, usize)).collect::<Vec<_>>()
                } else {
                    vec![]
                };

                (id, body)
            })
            .collect::<Vec<_>>();

        // Compute an action
        let mut rng = rand::thread_rng();
        let action = rng.gen_range(0..4);

        // You can output debug information to STDERR
        eprintln!("I am snake {} and I chose action {}", my_id, action);

        // Output the action
        println!("{}", action);
    }
}
