use core::panic;

fn make_map(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|v| match v {
                    '.' => 1,
                    '#' => 0,
                    '^' => Dir::North as i32,
                    _ => panic!("Unkwon Char {v}"),
                })
                .collect()
        })
        .collect()
}

fn find_start(map: &Vec<Vec<i32>>) -> Option<(usize, usize)> {
    for (i, row) in map.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if *val == 2 {
                return Some((i, j));
            }
        }
    }
    None
}

#[derive(Clone, Copy, PartialEq)]
enum Dir {
    North = 2,
    South = 3,
    East = 5,
    West = 7,
}

#[derive(PartialEq)]
enum MoveResult {
    Success(Dir, usize, usize),
    FailedBounds,
    FailedCycle,
}

fn check_bounds(
    dir: &Dir,
    curr_row: usize,
    curr_col: usize,
    max_row: usize,
    max_col: usize,
) -> bool {
    use Dir::*;
    match dir {
        South => curr_row < max_row,
        East => curr_col < max_col,
        North => curr_row > 0,
        West => curr_col > 0,
    }
}

fn turn(dir: Dir) -> Dir {
    use Dir::*;
    match dir {
        North => East,
        East => South,
        South => West,
        West => North,
    }
}

fn move_map(map: &mut Vec<Vec<i32>>, dir: Dir, row: usize, col: usize) -> MoveResult {
    use Dir::*;
    use MoveResult::*;

    if !check_bounds(&dir, row, col, map.len() - 1, map[row].len() - 1) {
        return FailedBounds;
    }

    let (new_row, new_col);
    match dir {
        South => {
            new_row = row + 1;
            new_col = col;
        }
        East => {
            new_row = row;
            new_col = col + 1;
        }
        North => {
            new_row = row - 1;
            new_col = col;
        }
        West => {
            new_row = row;
            new_col = col - 1;
        }
    }

    let next_val = map[new_row][new_col];
    map[new_row][new_col] *= dir as i32;

    if next_val == 0 {
        return Success(turn(dir), row, col);
    }

    if next_val % (dir as i32) == 0 {
        return FailedCycle;
    }

    Success(dir, new_row, new_col)
}

fn day06a(input: &str) -> i32 {
    let mut map = make_map(input);
    let Some((row, col)) = find_start(&map) else {
        panic!("Start Not Found")
    };

    let mut val = move_map(&mut map, Dir::North, row, col);
    while let MoveResult::Success(dir, row, col) = val {
        val = move_map(&mut map, dir, row, col)
    }

    map.iter()
        .map(|row| {
            row.iter()
                .fold(0, |acc, v| if *v > 1 { acc + 1 } else { acc })
        })
        .sum()
}

fn day06b(input: &str) -> i32 {
    let map_og = make_map(input);
    let Some((start_row, start_col)) = find_start(&map_og) else {
        panic!("Start Not Found")
    };

    let mut count = 0;
    for i in 0..map_og.len() {
        for j in 0..map_og[i].len() {
            if map_og[i][j] == 0 || map_og[i][j] == Dir::North as i32 {
                continue;
            }

            let mut map = map_og.clone();
            map[i][j] = 0;

            let mut val = move_map(&mut map, Dir::North, start_row, start_col);
            while let MoveResult::Success(dir, row, col) = val {
                val = move_map(&mut map, dir, row, col)
            }

            if val == MoveResult::FailedCycle {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    println!("{}", day06a(include_str!("../../input/day06.input")));
    println!("{}", day06b(include_str!("../../input/day06.input")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a() {
        let r = day06a(
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
        );
        assert_eq!(r, 41);
    }
}
