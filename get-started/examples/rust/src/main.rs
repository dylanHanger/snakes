pub mod macros;
use rand::Rng;

fn main() {
    #[cfg(debug_assertions)]
    {
        use std::process::Command;
        use std::time::Duration;

        // This block of code will only run when we are debugging.
        // It launches an instance of VS Code and attaches the debugger to the snake process.
        // This isn't a super nice way to do this, but unfortunately there isn't a better way that I know of.

        let url = format!(
            "vscode://vadimcn.vscode-lldb/launch/config?{{'request':'attach','pid':{}}}",
            std::process::id()
        );

        if cfg!(target_os = "windows") {
            Command::new("cmd")
                .arg("/C")
                .args(["code", "--open-url"])
                .arg(url)
                .output()
                .unwrap()
        } else {
            Command::new("sh")
                .arg("-c")
                .args(["code", "--open-url"])
                .arg(url)
                .output()
                .unwrap()
        };

        std::thread::sleep(Duration::from_millis(1000)); // Wait for debugger to attach
    }

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

        let num_snakes = read!(usize);
        let snakes: Vec<_> = (0..num_snakes)
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
