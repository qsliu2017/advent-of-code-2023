fn day1(input: &str) -> i32 {
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
fn test_day1() {
    let my_input = include_str!("../input/day1.example");
    let n = day1(my_input);
    assert_eq!(n, 142);
}
