use nom::{
    character::complete::u8 as nomu8,
    character::complete::{newline, space1},
    multi::separated_list1,
    IResult,
};

trait IsSafe: Iterator<Item = u8> {
    fn is_safe(&mut self) -> bool
    where
        Self: Sized,
    {
        let iter = self;
        let first = iter
            .next()
            .expect("Report must have at least two levels to be checked for safety");
        let second = iter
            .next()
            .expect("Report must have at least two levels to be checked for safety");
        let range = 1u8..4;
        if !range.contains(&first.abs_diff(second)) {
            return false;
        }
        let cmp = first.cmp(&second);
        let mut last = second;
        for next in iter {
            if !range.contains(&last.abs_diff(next)) || last.cmp(&next) != cmp {
                return false;
            }
            last = next;
        }
        true
    }
}

impl<I> IsSafe for I where I: Iterator<Item = u8> {}

trait SkipAny: Iterator<Item = u8> {
    fn skip_any(self) -> bool
    where
        Self: Sized,
    {
        let v = self.collect::<Vec<u8>>();
        (0..v.len()).any(|skip_index| {
            v.iter()
                .copied()
                .take(skip_index)
                .chain(v.iter().copied().skip(skip_index + 1))
                .is_safe()
        })
    }
}

impl<I> SkipAny for I where I: Iterator<Item = u8> {}

pub fn process_part1(input: &str) -> String {
    let (input, reports) = parse_input(input).unwrap();
    assert!(input.is_empty());
    reports
        .into_iter()
        .filter(|r| r.iter().copied().is_safe())
        .count()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (input, reports) = parse_input(input).unwrap();
    assert!(input.is_empty());
    reports
        .into_iter()
        .filter(|r| r.iter().copied().is_safe() || r.iter().copied().skip_any())
        .count()
        .to_string()
}

type Line<'a> = Vec<u8>;

fn parse_input(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, line) = separated_list1(space1, nomu8)(input)?;
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
        assert_eq!(result, "2");
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("./part1-example1.txt").unwrap();
        let result = process_part2(&input);
        assert_eq!(result, "4");
    }
}
