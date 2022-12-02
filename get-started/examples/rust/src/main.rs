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
    let my_id = read!(usize);

    loop {
        // Read the current game state
        // WARNING: This is just an example. You should use better data structures, such as your own structs and types.
        let num_apples = read!(usize);
        let apples = (0..num_apples)
            .map(|_| read!(usize, usize))
            .collect::<Vec<_>>();

        let num_snakes = read!(usize);
        let snakes = (0..num_snakes)
            .map(|_| {
                let id = read!(usize);
                let len = read!(usize);
                let body = (0..len).map(|_| read!(usize, usize)).collect::<Vec<_>>();

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
