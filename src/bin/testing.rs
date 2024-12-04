use itertools::Itertools;

fn main() {
    let dirs = vec![-1, -1, 1, 1, 0];
    let dirs = dirs.iter().permutations(2).unique();
    for ele in dirs {
        dbg!(ele);
    }
}
