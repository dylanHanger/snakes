#[macro_export]
macro_rules! readln {
    ($t: ty) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse::<$t>().unwrap()
        }
    };
    ($($t:ty),+ $(,)?) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.trim().split_whitespace();

            ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
        }
    };
}

#[macro_export]
macro_rules! read {
    ($t:ty) => {{
        use std::io::Read;
        let iter = std::io::stdin().bytes().map(std::result::Result::unwrap);

        let word = iter.map(|byte| byte as char)
            .skip_while(|c| c.is_whitespace())
            .take_while(|c| !c.is_whitespace())
            .collect::<String>();
        word.parse::<$t>()
            .unwrap_or_else(|e| panic!("Could not read requested {} \"{}\": {}", std::any::type_name::<$t>(), word, e))
    }};
    ($($t:ty),+ $(,)?) => {{
        ($(read!($t)),+)
    }};
}
