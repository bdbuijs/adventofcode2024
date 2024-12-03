use itertools::Itertools;

use nom::{
    character::complete::{newline, space1, u32 as nomu32},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (input, (mut first, mut second)) = parse_input(input).unwrap();
    assert!(input.is_empty());
    first.sort();
    second.sort();
    let total_difference = first
        .into_iter()
        .zip(second)
        .map(|(f, s)| f.abs_diff(s))
        .sum::<u32>();
    total_difference.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (input, (first, second)) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let second = second.into_iter().counts();
    let total = first
        .into_iter()
        .filter_map(|f| second.get(&f).map(|s| f * (*s as u32)))
        .sum::<u32>();
    total.to_string()
}

type Line<'a> = (u32, u32);

fn parse_input(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    let mut first = Vec::with_capacity(lines.len());
    let mut second = Vec::with_capacity(lines.len());
    lines.into_iter().for_each(|(f, s)| {
        first.push(f);
        second.push(s);
    });
    Ok((input, (first, second)))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, (first, _, second)) = tuple((nomu32, space1, nomu32))(input)?;
    let line = (first, second);
    Ok((input, line))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1() {
        let input = fs::read_to_string("./part1-example1.txt").unwrap();
        let result = process_part1(&input);
        assert_eq!(result, "11");
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("./part1-example1.txt").unwrap();
        let result = process_part2(&input);
        assert_eq!(result, "31");
    }
}
