use nom::{
    bytes::complete::tag,
    character::complete::newline,
    character::complete::{char as nomchar, u64 as nomu64},
    multi::separated_list1,
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (input, equations) = parse_input(input).unwrap();
    assert!(input.is_empty());
    equations
        .into_iter()
        .filter(|e| {
            let target = e.lhs;
            let current = e.rhs[0];
            evaluate(target, current, &e.rhs[1..])
        })
        .map(|e| e.lhs)
        .sum::<u64>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (input, equations) = parse_input(input).unwrap();
    assert!(input.is_empty());
    equations
        .into_iter()
        .filter(|e| {
            let target = e.lhs;
            let current = e.rhs[0];
            evaluate2(target, current, &e.rhs[1..])
        })
        .map(|e| e.lhs)
        .sum::<u64>()
        .to_string()
}

#[derive(Debug)]
struct Equation {
    lhs: u64,
    rhs: Vec<u64>,
}

impl Equation {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, lhs) = nomu64(input)?;
        let (input, _) = tag(": ")(input)?;
        let (input, rhs) = separated_list1(nomchar(' '), nomu64)(input)?;
        Ok((input, Self { lhs, rhs }))
    }
}

fn evaluate(target: u64, current: u64, tail: &[u64]) -> bool {
    if tail.is_empty() {
        return target == current;
    }
    let next_number = tail[0];
    let plus = current + next_number;
    let mult = current * next_number;
    evaluate(target, plus, &tail[1..]) || evaluate(target, mult, &tail[1..])
}

fn evaluate2(target: u64, current: u64, tail: &[u64]) -> bool {
    if tail.is_empty() {
        return target == current;
    }
    let next_number = tail[0];
    let plus = current + next_number;
    let mult = current * next_number;
    let conc = concatenate(current, next_number);
    evaluate2(target, plus, &tail[1..])
        || evaluate2(target, mult, &tail[1..])
        || evaluate2(target, conc, &tail[1..])
}

fn concatenate(a: u64, b: u64) -> u64 {
    let zeroes = b.ilog10() + 1;
    a * 10_u64.pow(zeroes) + b
}

fn parse_input(input: &str) -> IResult<&str, Vec<Equation>> {
    let (input, lines) = separated_list1(newline, Equation::parse)(input)?;
    Ok((input, lines))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1() {
        let input = fs::read_to_string("./part1-example1.txt").unwrap();
        let result = process_part1(&input);
        assert_eq!(result, "3749");
    }

    #[test]
    fn test_concatenate() {
        assert_eq!(concatenate(1234, 5678), 12345678)
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("./part1-example1.txt").unwrap();
        let result = process_part2(&input);
        assert_eq!(result, "11387");
    }
}
