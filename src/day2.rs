fn part1(input: &str) -> i32 {
    input
        .lines()
        .into_iter()
        .enumerate()
        .map(|(idx, line)| {
            (
                idx + 1,
                line.split_once(":").unwrap().1.split(";").all(|set| {
                    set.split(",").all(|color| {
                        let (n, color) = color.trim().split_once(" ").unwrap();
                        let n = n.parse::<i32>().unwrap();
                        match color {
                            "red" => n <= 12,
                            "green" => n <= 13,
                            "blue" => n <= 14,
                            _ => n <= 0,
                        }
                    })
                }),
            )
        })
        .inspect(|(idx, ok)| {
            dbg!((idx, ok));
        })
        .filter(|&(_, ok)| ok)
        .map(|(idx, _)| idx as i32)
        .sum()
}
#[test]
fn test_part1() {
    let input = include_str!("../input/day2_part1.example");
    let n = part1(input);
    assert_eq!(n, 8);
}
