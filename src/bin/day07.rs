use itertools::{repeat_n, Itertools};

fn day07a(input: &str) -> u64 {
    input
        .lines()
        .filter_map(|line| {
            let (target, values) = line.split_once(':').unwrap();
            let target: u64 = target.parse().unwrap();

            let values: Vec<u64> = values
                .trim()
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect();
            // println!("{target}: {values:?}");

            let num_opts = values.len() - 1;
            let options = repeat_n(0..=1, num_opts).multi_cartesian_product();

            'outer: for opt in options {
                // println!("Trying: {opt:?}");
                let mut curr = values[0];
                for (oper, val) in opt.iter().zip(values.iter().skip(1)) {
                    match oper {
                        0 => curr += val,
                        1 => curr *= val,
                        _ => unreachable!(),
                    };
                    if curr > target {
                        continue 'outer;
                    };
                }
                if curr == target {
                    // println!("Found");
                    return Some(target);
                }
            }
            None
        })
        .sum()
}

fn day07b(input: &str) -> u64 {
    input
        .lines()
        .filter_map(|line| {
            let (target, values) = line.split_once(':').unwrap();
            let target: u64 = target.parse().unwrap();

            let values: Vec<u64> = values
                .trim()
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect();
            // println!("{target}: {values:?}");

            let num_opts = values.len() - 1;
            let options = repeat_n(0..=2, num_opts).multi_cartesian_product();

            'outer: for opt in options {
                // println!("Trying: {opt:?}");
                let mut curr = values[0];
                for (oper, val) in opt.iter().zip(values.iter().skip(1)) {
                    match oper {
                        0 => curr += val,
                        1 => curr *= val,
                        2 => curr = format!("{curr}{val}").parse().unwrap(),
                        _ => unreachable!(),
                    };
                    if curr > target {
                        continue 'outer;
                    };
                }
                if curr == target {
                    // println!("Found");
                    return Some(target);
                }
            }
            None
        })
        .sum()
}

fn main() {
    println!("{}", day07a(include_str!("../../input/day07.input")));
    println!("{}", day07b(include_str!("../../input/day07.input")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let r = day07a(input);
        assert_eq!(r, 3749);
    }

    #[test]
    fn part_2() {
        let input = "";
        let r = day07b(input);
        assert_eq!(r, 0);
    }
}
