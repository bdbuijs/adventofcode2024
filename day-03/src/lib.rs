use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{anychar, u32 as nomu32},
    combinator::{eof, recognize, value},
    multi::many1,
    sequence::{terminated, tuple},
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (input, muls) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let total = muls.into_iter().map(|(a, b)| a * b).sum::<u32>();
    total.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (input, muls) = parse_input2(input).unwrap();
    assert!(input.is_empty());
    let total = muls.into_iter().map(|(a, b)| a * b).sum::<u32>();
    total.to_string()
}

fn parse_input(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    let (input, muls) = many1(alt((parse_mul, value(None, anychar))))(input)?;
    let muls = muls.into_iter().flatten().collect();
    Ok((input, muls))
}

fn parse_input2(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    let (input, muls) = many1(alt((parse_dont, parse_mul, value(None, anychar))))(input)?;
    let muls = muls.into_iter().flatten().collect();
    Ok((input, muls))
}

fn parse_dont(input: &str) -> IResult<&str, Option<(u32, u32)>> {
    let (input, _) = tuple((
        tag("don't()"),
        alt((
            terminated(take_until("do()"), tag("do()")),
            recognize(terminated(many1(anychar), eof)),
        )),
    ))(input)?;
    Ok((input, None))
}

fn parse_mul(input: &str) -> IResult<&str, Option<(u32, u32)>> {
    let (input, _) = tag("mul(")(input)?;
    let (input, lhs) = nomu32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, rhs) = nomu32(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, Some((lhs, rhs))))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1() {
        let input = fs::read_to_string("./part1-example1.txt").unwrap();
        let result = process_part1(&input);
        assert_eq!(result, "161");
    }

    #[test]
    fn dont() {
        let input = fs::read_to_string("./part2-dont.txt").unwrap();
        let result = process_part2(&input);
        assert_eq!(result, "1");
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("./part2-example1.txt").unwrap();
        let result = process_part2(&input);
        assert_eq!(result, "48");
    }
}
