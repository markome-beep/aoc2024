use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    for argument in env::args().skip(1) {
        let f = format!("./src/bin/day{}.rs", argument);
        let mut file = OpenOptions::new().write(true).create_new(true).open(f)?;
        let content = format!(
            "fn day{}a(input: &str) -> i32 {{
    0
}}

fn day{}b(input: &str) -> i32 {{
    0
}}

fn main() {{
    println!(\"{{}}\", day{}a(include_str!(\"../../input/day{}.input\")));
    println!(\"{{}}\", day{}b(include_str!(\"../../input/day{}.input\")));
}}

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn part_1() {{
        let input = \"\";
        let r = day{}a(input);
        assert_eq!(r, 0);
    }}

    #[test]
    fn part_2() {{
        let input = \"\";
        let r = day{}b(input);
        assert_eq!(r, 0);
    }}
}}",
            argument, argument, argument, argument, argument, argument, argument, argument
        );
        file.write_all(content.as_bytes())?;
    }
    Ok(())
}
