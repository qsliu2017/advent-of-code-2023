use std::collections::HashSet;

fn day4part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let (winnings, numbers) = line.split_once(":").unwrap().1.split_once("|").unwrap();
            let winnings = winnings
                .trim()
                .split_ascii_whitespace()
                .map(|i| i.parse::<i32>().unwrap())
                .collect::<HashSet<_>>();
            let n = numbers
                .trim()
                .split_ascii_whitespace()
                .map(|i| i.parse::<i32>().unwrap())
                .filter(|i| winnings.contains(i))
                .count() as u32;
            n.eq(&0).then_some(0).unwrap_or_else(|| 1 << (n - 1))
        })
        .sum()
}

#[test]
fn test_day4part1() {
    let input = include_str!("../input/day4.example");
    let n = day4part1(input);
    assert_eq!(n, 13);
}
