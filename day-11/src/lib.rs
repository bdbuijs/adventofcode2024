use std::collections::HashMap;

use nom::{
    character::complete::space1, character::complete::u64 as nomu64, multi::separated_list1,
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (input, mut stones) = parse_input(input).unwrap();
    assert!(input.is_empty());
    (0..25).for_each(|_| {
        stones = stones.iter().copied().flat_map(rules).collect();
    });
    stones.len().to_string()
}

pub fn process_part2(input: &str) -> String {
    let (input, stones) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let mut stones: HashMap<u64, u64> = stones.into_iter().map(|n| (n, 1)).collect();
    (0..75).for_each(|_| {
        blink(&mut stones);
    });
    stones.into_values().sum::<u64>().to_string()
}

fn rules(n: u64) -> impl Iterator<Item = u64> {
    let new_stones = {
        if n == 0 {
            [Some(1), None]
        } else {
            let digits = n.ilog10() + 1;
            if digits % 2 == 0 {
                let pow10 = 10_u64.pow(digits / 2);
                [Some(n / pow10), Some(n % pow10)]
            } else {
                [Some(n * 2024), None]
            }
        }
    };
    new_stones.into_iter().flatten()
}

fn blink(stones: &mut HashMap<u64, u64>) {
    let mut new_stones = HashMap::new();
    stones.drain().for_each(|(k, v)| {
        rules(k).for_each(|new_stone| {
            new_stones
                .entry(new_stone)
                .and_modify(|new_v| *new_v += v)
                .or_insert(v);
        })
    });

    std::mem::swap(stones, &mut new_stones);
}

fn parse_input(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, stones) = separated_list1(space1, nomu64)(input)?;
    Ok((input, stones))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1() {
        let input = fs::read_to_string("./part1-example1.txt").unwrap();
        let result = process_part1(&input);
        assert_eq!(result, "55312");
    }
}
