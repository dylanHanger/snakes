mod macros;

pub fn get_codelldb_url() -> String {
    format!(
        "vscode://vadimcn.vscode-lldb/launch/config?{{'request':'attach','pid':{}}}",
        std::process::id()
    )
}

pub fn attach_vscode() {
    let url = get_codelldb_url();
    // Launch vscode at the requested URL
    if std::env::consts::OS == "windows" {
        std::process::Command::new("cmd")
            .args(["/C", "start", &url])
            .spawn()
            .expect("Failed to launch vscode");
    } else {
        std::process::Command::new("open")
            .arg(url)
            .spawn()
            .expect("Failed to launch vscode");
    };
}

fn main() {
    #[cfg(debug_assertions)]
    {
        use std::env::args;

        let mut args = args();
        if args.any(|arg| arg == "--attach-debugger") {
            // Note that this requires `https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb` to be installed, and only works on VSCode.
            attach_vscode();
            eprintln!(
                "Waiting for debugger to attach to PID {}",
                std::process::id()
            );
            std::thread::sleep(std::time::Duration::from_secs(5))
        }
    }

    // Game initialization
    let (game_width, game_height) = read!(usize, usize);
    let (food_lifetime_raw, food_value) = read!(usize, f32);
    let (num_snakes, my_id) = read!(usize, u32);
    let (max_turns, timeout_raw) = read!(usize, i32);
    let food_lifetime = Some(food_lifetime_raw).filter(|&x| x > 0).map(|x| x as f32);
    let timeout = Some(timeout_raw).filter(|&x| x > 0).map(|x| x as usize);

    eprintln!("I am #{my_id} of {num_snakes}.");
    eprintln!(
        "Food is worth {food_value} and lasts {0}.",
        food_lifetime
            .map(|l| format!("for {l} turns"))
            .unwrap_or("forever".to_string())
    );
    eprintln!("The board size is {game_width}x{game_height}.");
    eprintln!(
        "There are {max_turns} turns, and I have {0} to make each move",
        timeout
            .map(|t| format!("{t}ms"))
            .unwrap_or("infinite time".to_string())
    );

    // Update loop
    let mut current_turn = 0;
    loop {
        current_turn += 1;
        eprintln!("Turn {current_turn}");

        // Read the state of the game
        let num_food = read!(usize);
        for _ in 0..num_food {
            let (lifetime, x, y) = read!(f32, u32, u32);
            if let Some(food_lifetime) = food_lifetime {
                let value = (food_value * (lifetime / food_lifetime * 2. - 1.)).round();
                eprintln!(
                    "The food at ({x},{y}) has {lifetime} turns remaining. It is worth {value}"
                )
            } else {
                eprintln!("The food at ({x},{y}) is worth {food_value} and will not rot.")
            }
        }

        for _ in 0..num_snakes {
            let id = read!(u32);
            let (kills, deaths) = read!(i32, u32);
            let length = read!(usize);
            for i in 0..length {
                let (x, y) = read!(usize, usize);
                if i == 0 {
                    eprintln!(
                        "Snake #{id} starts at ({x},{y}), is {length} long, and has a K/D of {kills}/{deaths}"
                    )
                }
            }
        }

        // Output your move on stdout
        const DIRECTIONS: [&str; 6] = ["north", "east", "east", "south", "west", "west"];
        let direction = DIRECTIONS[current_turn % DIRECTIONS.len()];
        eprintln!("I am going to move {direction}");
        println!("{}", direction);
        eprintln!("================================")
    }
}
