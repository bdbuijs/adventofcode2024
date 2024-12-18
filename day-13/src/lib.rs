use nom::{
    bytes::complete::tag,
    character::complete::{i64 as nomi64, newline},
    multi::separated_list1,
    sequence::pair,
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (input, machines) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let total_tokens = machines.into_iter().map(tokens).sum::<i64>();
    total_tokens.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (input, machines) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let total_tokens = machines.into_iter().map(tokens2).sum::<i64>();
    total_tokens.to_string()
}

#[derive(Debug)]
struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

impl Machine {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("Button A: X+")(input)?;
        let (input, ax) = nomi64(input)?;
        let (input, _) = tag(", Y+")(input)?;
        let (input, ay) = nomi64(input)?;
        let (input, _) = tag("\nButton B: X+")(input)?;
        let (input, bx) = nomi64(input)?;
        let (input, _) = tag(", Y+")(input)?;
        let (input, by) = nomi64(input)?;
        let (input, _) = tag("\nPrize: X=")(input)?;
        let (input, px) = nomi64(input)?;
        let (input, _) = tag(", Y=")(input)?;
        let (input, py) = nomi64(input)?;
        let machine = Machine {
            a: (ax, ay),
            b: (bx, by),
            prize: (px, py),
        };
        Ok((input, machine))
    }
}

fn tokens(machine: Machine) -> i64 {
    let (u, v) = machine.a;
    let (w, z) = machine.b;
    let (c, d) = machine.prize;
    let zcwd = (z * c) - (w * d);
    let uzvw = (u * z) - (v * w);
    if zcwd % uzvw == 0 {
        let a = zcwd / uzvw;
        let cua = c - u * a;
        if cua % w == 0 {
            let b = cua / w;
            return a * 3 + b;
        }
    }
    0
}

fn tokens2(machine: Machine) -> i64 {
    let (u, v) = machine.a;
    let (w, z) = machine.b;
    let (c, d) = machine.prize;
    let c = c + 10_000_000_000_000;
    let d = d + 10_000_000_000_000;
    let zcwd = (z * c) - (w * d);
    let uzvw = (u * z) - (v * w);
    if zcwd % uzvw == 0 {
        let a = zcwd / uzvw;
        let cua = c - u * a;
        if cua % w == 0 {
            let b = cua / w;
            return a * 3 + b;
        }
    }
    0
}

fn parse_input(input: &str) -> IResult<&str, Vec<Machine>> {
    let (input, lines) = separated_list1(pair(newline, newline), Machine::parse)(input)?;
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
        assert_eq!(result, "480");
    }
}
