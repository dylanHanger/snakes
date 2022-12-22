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
