use std::collections::{HashMap, HashSet};

fn day3part1(input: &str) -> i32 {
    let (symbols, numbers) = input.lines().into_iter().enumerate().fold(
        (
            HashSet::<(usize, usize)>::new(),
            HashMap::<(usize, usize, usize), i32>::new(),
        ),
        |(symbols, numbers), (row, line)| {
            let (symbols, numbers, _) = (line.to_owned() + ".")
                .as_bytes()
                .into_iter()
                .enumerate()
                .fold(
                    (symbols, numbers, String::new()),
                    |(mut symbols, mut numbers, mut stack), (col, b)| {
                        if b.is_ascii_digit() {
                            stack.push(*b as char);
                        } else {
                            if stack.len() > 0 {
                                let n = stack.parse::<i32>().unwrap();
                                numbers.insert((row, col - stack.len(), col - 1), n);
                            }
                            if *b != b'.' {
                                symbols.insert((row, col));
                            }
                            stack.clear();
                        }
                        (symbols, numbers, stack)
                    },
                );
            (symbols, numbers)
        },
    );
    numbers
        .into_iter()
        .filter_map(|((row, start, end), n)| {
            (start.eq(&0).then(|| 0).unwrap_or_else(|| start - 1)..=end + 1)
                .flat_map(|col| {
                    (row.eq(&0).then(|| 0).unwrap_or_else(|| row - 1)..=row + 1)
                        .map(move |row| (row, col))
                })
                .any(|(row, col)| symbols.contains(&(row, col)))
                .then_some(n)
        })
        .sum()
}

#[test]
fn test_day3part1() {
    let input = include_str!("../input/day3_part1.example");
    let n = day3part1(input);
    assert_eq!(n, 4361);
}

fn day3part2(input: &str) -> i32 {
    let (mut symbols, numbers) = input.lines().into_iter().enumerate().fold(
        (
            HashMap::<(usize, usize), Vec<i32>>::new(),
            HashMap::<(usize, usize, usize), i32>::new(),
        ),
        |(symbols, numbers), (row, line)| {
            let (symbols, numbers, _) = (line.to_owned() + ".")
                .as_bytes()
                .into_iter()
                .enumerate()
                .fold(
                    (symbols, numbers, String::new()),
                    |(mut symbols, mut numbers, mut stack), (col, b)| {
                        if b.is_ascii_digit() {
                            stack.push(*b as char);
                        } else {
                            if stack.len() > 0 {
                                let n = stack.parse::<i32>().unwrap();
                                numbers.insert((row, col - stack.len(), col - 1), n);
                            }
                            if *b == b'*' {
                                symbols.insert((row, col), Vec::new());
                            }
                            stack.clear();
                        }
                        (symbols, numbers, stack)
                    },
                );
            (symbols, numbers)
        },
    );
    numbers.into_iter().for_each(|((row, start, end), n)| {
        (start.eq(&0).then(|| 0).unwrap_or_else(|| start - 1)..=end + 1)
            .flat_map(|col| {
                (row.eq(&0).then(|| 0).unwrap_or_else(|| row - 1)..=row + 1)
                    .map(move |row| (row, col))
            })
            .for_each(|(row, col)| {
                symbols.get_mut(&(row, col)).map(|v| v.push(n));
            });
    });
    symbols
        .into_iter()
        .filter_map(|(_, adjs)| adjs.len().eq(&2).then(|| adjs.into_iter().product::<i32>()))
        .sum()
}

#[test]
fn test_day3part2() {
    let input = include_str!("../input/day3_part1.example");
    let n = day3part2(input);
    assert_eq!(n, 467835);
    // println!("{n}")
}
