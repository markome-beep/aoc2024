use std::{collections::HashMap, usize};

use itertools::Itertools;
use regex::Regex;

pub fn day01a(input: &str) -> i32 {
    let (mut vl, mut vr): (Vec<i32>, Vec<i32>) = (vec![], vec![]);

    input.split('\n').for_each(|line| {
        let Some((l, r)) = line.split_once("   ") else {
            return;
        };
        vl.push(l.parse().expect("Could not read number"));
        vr.push(r.parse().expect("Could not read number"));
    });

    vl.sort();
    vr.sort();

    vl.into_iter()
        .zip(vr.into_iter())
        .fold(0, |acc, (l, r)| acc + (l - r).abs())
}

pub fn day01b(input: &str) -> i32 {
    let mut vl: Vec<i32> = vec![];
    let mut vr: HashMap<i32, i32> = HashMap::new();

    input.split('\n').for_each(|line| {
        let Some((l, r)) = line.split_once("   ") else {
            return;
        };
        vl.push(l.parse().expect("Could not read number"));
        *vr.entry(r.parse().expect("Could not read number"))
            .or_default() += 1;
    });

    vl.into_iter()
        .fold(0, |acc, l| acc + l * vr.get(&l).unwrap_or(&0))
}

fn day02_help(nums: &[i32]) -> bool {
    let f = nums[0] - nums[1];

    if f.abs() == 0 || f.abs() >= 4 {
        return false;
    }
    let f = f / f.abs();

    nums[..]
        .windows(2)
        .map(|s| (s[0] - s[1]) * f)
        .all(|i| i > 0 && i < 4)
}

pub fn day02a(input: &str) -> i32 {
    input.split('\n').fold(0, |acc, line| {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse().expect("NOPE"))
            .collect();

        if nums.len() == 0 {
            return acc;
        }
        if day02_help(&nums[..]) {
            return acc + 1;
        }
        acc
    })
}

pub fn day02b(input: &str) -> i32 {
    input.split('\n').fold(0, |acc, line| {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse().expect("NOPE"))
            .collect();

        if nums.len() == 0 {
            return acc;
        }

        let r = day02_help(&nums[..]);
        if r {
            return acc + 1;
        } else if (0..nums.len()).any(|a| day02_help(&[&nums[0..a], &nums[a + 1..]].concat())) {
            return acc + 1;
        }

        acc
    })
}

pub fn day03a(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d*),(\d*)\)").unwrap();

    re.captures_iter(input)
        .map(|caps| {
            let (_, [l, r]) = caps.extract();
            l.parse::<i32>().unwrap() * r.parse::<i32>().unwrap()
        })
        .sum()
}

pub fn day03b(input: &str) -> i32 {
    let mut chars = input.chars().enumerate().peekable();
    let mut enabled = true;
    let mut sum = 0;

    while let Some((i, c)) = chars.next() {
        if i + 8 > input.len() {
            break;
        }
        if c == 'd' {
            if &input[i..i + 4] == "do()" {
                chars.nth(2);
                enabled = true;
            } else if &input[i..i + 7] == "don't()" {
                chars.nth(5);
                enabled = false;
            }
        } else if enabled && c == 'm' {
            if &input[i..i + 4] == "mul(" {
                chars.nth(2);
            } else {
                continue;
            }
            let n1: i32 = chars
                .by_ref()
                .map(|(_, d)| d)
                .take_while(|d| d.is_numeric())
                .collect::<String>()
                .parse()
                .unwrap_or(0);

            let Some((i, _)) = chars.peek() else {
                continue;
            };

            if input.chars().nth(i - 1).unwrap() != ',' {
                continue;
            }
            let n2: i32 = chars
                .by_ref()
                .map(|(_, d)| d)
                .take_while(|d| d.is_numeric())
                .collect::<String>()
                .parse()
                .unwrap_or(0);

            let Some((i, _)) = chars.peek() else {
                continue;
            };

            if input.chars().nth(i - 1).unwrap() != ')' {
                continue;
            }
            sum += n1 * n2;
        }
    }
    sum
}

fn day04_mat(input: &str, b: bool) -> Vec<Vec<u8>> {
    if b {
        return input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        'X' => 0,
                        'M' => 2,
                        'A' => 1,
                        'S' => 4,
                        _ => 0,
                    })
                    .collect()
            })
            .collect();
    }
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'X' => 1,
                    'M' => 2,
                    'A' => 3,
                    'S' => 4,
                    _ => 0,
                })
                .collect()
        })
        .collect()
}

fn check_dir(mat: &Vec<Vec<u8>>, row: usize, col: usize, dir: (i32, i32), n: u8) -> bool {
    if mat[row][col] != n {
        return false;
    }
    if n == 4 {
        return true;
    }
    let (new_row, new_col) = (row as i32 + dir.0, col as i32 + dir.1);
    if new_row < 0 || new_row as usize >= mat.len() {
        return false;
    }
    let new_row = new_row as usize;
    if new_col < 0 || new_col as usize >= mat[new_row].len() {
        return false;
    }
    let new_col = new_col as usize;

    check_dir(mat, new_row, new_col, dir, n + 1)
}

pub fn day04a(input: &str) -> u64 {
    let ws = day04_mat(input, false);
    let dirs: Vec<i32> = vec![-1, -1, 1, 1, 0];
    let dirs = dirs.iter().permutations(2).unique();

    let mut sum = 0;
    for (i, row) in ws.iter().enumerate() {
        for (j, _col) in row.iter().enumerate() {
            for dir in dirs.clone() {
                if check_dir(&ws, i, j, (*dir[0], *dir[1]), 1) {
                    sum += 1;
                }
            }
        }
    }
    sum
}

fn check_diag(mat: &Vec<Vec<u8>>, row: usize, col: usize) -> bool {
    mat[row - 1][col - 1] + mat[row + 1][col + 1] == 6
        && mat[row + 1][col - 1] + mat[row - 1][col + 1] == 6
}

pub fn day04b(input: &str) -> u64 {
    let ws = day04_mat(input, true);

    let mut sum = 0;
    for (i, row) in ws[1..ws.len() - 1].iter().enumerate() {
        for (j, val) in row[1..row.len() - 1].iter().enumerate() {
            if *val == 1 && check_diag(&ws, i + 1, j + 1) {
                sum += 1;
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day02b() {
        let r = day02b(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        );
        assert_eq!(r, 4);
    }

    #[test]
    fn test_day03a() {
        let r = day03a("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(r, 161)
    }

    #[test]
    fn test_day03b() {
        let r = day03b("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(r, 161)
    }

    #[test]
    fn test_day03b_2() {
        let r = day03b("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(r, 48)
    }
    #[test]
    fn test_day04a() {
        let r = day04a(
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        );
        assert_eq!(r, 18)
    }

    #[test]
    fn test_day04b() {
        let r = day04b(
            ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........",
        );
        assert_eq!(r, 9)
    }
}
