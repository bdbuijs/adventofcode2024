use std::fs;

use itertools::Itertools;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        alpha1, char as nomchar, newline, one_of, space0, space1, u32 as nomu32,
    },
    combinator::recognize,
    multi::{many1, separated_list1},
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
        .zip(second.into_iter())
        .map(|(f, s)| f.abs_diff(s))
        .sum::<u32>();
    total_difference.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (input, (mut first, mut second)) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let second = second.into_iter().counts();
    let total = first
        .into_iter()
        .filter_map(|f| {
            if let Some(s) = second.get(&f) {
                Some(f * (*s as u32))
            } else {
                None
            }
        })
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
