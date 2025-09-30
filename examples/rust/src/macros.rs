mod input {
    #[macro_export]
    macro_rules! readln {
        ($($t:ty),+ $(,)?) => {{
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).expect("failed to read line");
            let mut iter = line.split_whitespace();

            (
                $(
                    iter.next()
                        .expect("not enough input")
                        .parse::<$t>()
                        .expect(&format!("failed to parse {} from input", std::any::type_name::<$t>()))
                ),+
            )
        }};
    }

    #[macro_export]
    macro_rules! read {
        ($($t:ty),+ $(,)?) => {{
            use std::io::Read;
            let stdin = std::io::stdin();
            let mut iter = stdin.lock().bytes().map(|res| res.expect("failed to read byte") as char);

            (
                $(
                    {
                        let word: String = iter
                            .by_ref()
                            .skip_while(|c| c.is_whitespace())
                            .take_while(|c| !c.is_whitespace())
                            .collect();

                        word.parse::<$t>()
                            .expect(&format!("could not parse {} from input '{}'", std::any::type_name::<$t>(), word))
                    }
                ),+
            )
        }};
    }
}
