fn day09a(input: &str) -> i32 {
    0
}

fn day09b(input: &str) -> i32 {
    0
}

fn main() {
    println!("{}", day09a(include_str!("../../input/day09.input")));
    println!("{}", day09b(include_str!("../../input/day09.input")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "";
        let r = day09a(input);
        assert_eq!(r, 0);
    }

    #[test]
    fn part_2() {
        let input = "";
        let r = day09b(input);
        assert_eq!(r, 0);
    }
}