fn part1(input: &str) -> i32 {
    input
        .split_ascii_whitespace()
        .map(|line| {
            let (first, last) =
                line.as_bytes()
                    .into_iter()
                    .fold((None, None), |(first, last), b| {
                        let n = b.is_ascii_digit().then(|| (b - b'0') as i32);
                        (first.or(n), n.or(last))
                    });
            dbg!(first.unwrap() * 10 + last.unwrap())
        })
        .sum()
}

#[test]
fn test_part1() {
    let my_input = include_str!("../input/day1_part1.example");
    let n = part1(my_input);
    assert_eq!(n, 142);
}

fn part2(input: &str) -> i32 {
    input
        .split_ascii_whitespace()
        .map(|line| {
            let (first, last) = [
                ("one", "one1one"),
                ("two", "two2two"),
                ("three", "three3three"),
                ("four", "four4four"),
                ("five", "five5five"),
                ("six", "six6six"),
                ("seven", "seven7seven"),
                ("eight", "eight8eight"),
                ("nine", "nine9nine"),
            ]
            .into_iter()
            .fold(line.to_string(), |s, (from, to)| s.replace(from, to))
            .as_bytes()
            .into_iter()
            .fold((None, None), |(first, last), b| {
                let n = b.is_ascii_digit().then(|| (b - b'0') as i32);
                (first.or(n), n.or(last))
            });
            dbg!(first.unwrap() * 10 + last.unwrap())
        })
        .sum()
}

#[test]
fn test_part2() {
    let input = include_str!("../input/day1_part2.example");
    let n = part2(input);
    assert_eq!(n, 281);
}
