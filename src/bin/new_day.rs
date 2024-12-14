use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    for argument in env::args().skip(1) {
        let f = format!("./src/bin/day{}.rs", argument);
        let mut file = OpenOptions::new().write(true).create_new(true).open(f)?;
        let content = format!(
            "use rust::{{day{}a, day{}b}};

fn main() {{
    println!(\"{{}}\", day{}a(include_str!(\"../../input/day{}.input\")));
    println!(\"{{}}\", day{}b(include_str!(\"../../input/day{}.input\")));
}}",
            argument, argument, argument, argument, argument, argument
        );
        file.write_all(content.as_bytes())?;
    }
    Ok(())
}
