use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

fn make_rules(rules_str: &str) -> HashMap<i32, HashSet<i32>> {
    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    rules_str.lines().for_each(|line| {
        let Some((left, right)) = line.split_once("|") else {
            panic!("RIP THIS LINE {line}");
        };

        rules
            .entry(left.parse().unwrap())
            .or_insert(HashSet::new())
            .insert(right.parse().unwrap());
    });
    rules
}

fn day05a(input: &str) -> i32 {
    let Some((rules_str, updates)) = input.split_once("\n\n") else {
        panic!("OH NO");
    };

    let rules = make_rules(rules_str);

    updates
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|line| {
            line.iter().enumerate().all(|(i, left)| {
                line.iter()
                    .skip(i + 1)
                    .all(|right| rules.get(left).unwrap_or(&HashSet::new()).contains(right))
            })
        })
        .map(|line| line[line.len() / 2])
        .sum()
}

#[derive(Debug, Eq, PartialEq)]
struct Num<'a> {
    value: i32,
    rules_ref: &'a HashMap<i32, HashSet<i32>>,
}

impl Ord for Num<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self
            .rules_ref
            .get(&self.value)
            .unwrap_or(&HashSet::new())
            .contains(&other.value)
        {
            Ordering::Less
        } else if other
            .rules_ref
            .get(&other.value)
            .unwrap_or(&HashSet::new())
            .contains(&self.value)
        {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Num<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn day05b(input: &str) -> i32 {
    let Some((rules_str, updates)) = input.split_once("\n\n") else {
        panic!("OH NO");
    };

    let rules = make_rules(rules_str);

    updates
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|line| {
            !line.iter().enumerate().all(|(i, left)| {
                line.iter()
                    .skip(i + 1)
                    .all(|right| rules.get(left).unwrap_or(&HashSet::new()).contains(right))
            })
        })
        .map(|line| {
            line.iter()
                .map(|n| Num {
                    value: *n,
                    rules_ref: &rules,
                })
                .collect::<Vec<Num>>()
        })
        .map(|mut line| {
            line.sort();
            line[line.len() / 2].value
        })
        .sum()
}

fn main() {
    println!("{}", day05a(include_str!("../../input/day05.input")));
    println!("{}", day05b(include_str!("../../input/day05.input")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parta() {
        let r = day05a(
            "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
        );
        dbg!(&r);
        assert_eq!(r, 143)
    }

    #[test]
    fn partb() {
        let r = day05b(
            "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
        );
        dbg!(&r);
        assert_eq!(r, 123)
    }
}
