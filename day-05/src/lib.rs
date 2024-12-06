use std::collections::HashMap;

use nom::{
    character::complete::{char as nomchar, newline, u8 as nomu8},
    multi::separated_list1,
    sequence::{pair, terminated, tuple},
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (input, (rules, updates)) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let rules: HashMap<u8, Vec<u8>> =
        rules
            .into_iter()
            .fold(HashMap::new(), |mut acc, (before, after)| {
                acc.entry(after)
                    .and_modify(|v| v.push(before))
                    .or_insert(vec![before]);
                acc
            });
    let count: usize = updates
        .into_iter()
        .filter_map(|pages| {
            if is_correctly_ordered(&pages, &rules) {
                Some(pages[pages.len() / 2] as usize)
            } else {
                None
            }
        })
        .sum();
    count.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (input, (rules, updates)) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let rules_map: HashMap<u8, Vec<u8>> =
        rules
            .iter()
            .copied()
            .fold(HashMap::new(), |mut acc, (before, after)| {
                acc.entry(after)
                    .and_modify(|v| v.push(before))
                    .or_insert(vec![before]);
                acc
            });
    updates
        .into_iter()
        .filter(|pages| !is_correctly_ordered(pages, &rules_map))
        .map(|mut pages| {
            loop {
                let mut made_change = false;
                rules.iter().copied().for_each(|rule| {
                    if let Some((before_index, after_index)) = find_rule(rule, &pages) {
                        pages.swap(before_index, after_index);
                        made_change = true;
                    }
                });
                if !made_change {
                    break;
                }
            }
            pages[pages.len() / 2] as u64
        })
        .sum::<u64>()
        .to_string()
}

/// Returns Some(index of 'before', index of 'after') if rule is applicable, but not followed
fn find_rule(rule: (u8, u8), pages: &[u8]) -> Option<(usize, usize)> {
    let (before, after) = rule;
    match (
        pages.iter().enumerate().find(|&(_i, &n)| n == before),
        pages.iter().enumerate().find(|&(_i, &n)| n == after),
    ) {
        (Some((before_i, _before_n)), Some((after_i, _after_n))) if before_i > after_i => {
            Some((before_i, after_i))
        }
        _ => None,
    }
}

fn is_correctly_ordered(pages: &Vec<u8>, rules: &HashMap<u8, Vec<u8>>) -> bool {
    let mut must_not_encounter = Vec::new();
    for &page in pages {
        if must_not_encounter.contains(&page) {
            return false;
        }
        if let Some(must_occur_before) = rules.get(&page) {
            must_not_encounter.extend_from_slice(must_occur_before);
        }
    }
    true
}

type Rules = Vec<(u8, u8)>;
type Updates = Vec<Vec<u8>>;

fn parse_input(input: &str) -> IResult<&str, (Rules, Updates)> {
    let (input, rules) = parse_rules(input)?;
    let (input, updates) = parse_pages(input)?;
    Ok((input, (rules, updates)))
}

fn parse_rules(input: &str) -> IResult<&str, Vec<(u8, u8)>> {
    let (input, rules) =
        terminated(separated_list1(newline, parse_rule), pair(newline, newline))(input)?;
    Ok((input, rules))
}

fn parse_rule(input: &str) -> IResult<&str, (u8, u8)> {
    let (input, (a, _, b)) = tuple((nomu8, nomchar('|'), nomu8))(input)?;
    Ok((input, (a, b)))
}

fn parse_pages(input: &str) -> IResult<&str, Vec<Vec<u8>>> {
    let (input, pages) = separated_list1(newline, parse_page)(input)?;
    Ok((input, pages))
}

fn parse_page(input: &str) -> IResult<&str, Vec<u8>> {
    let (input, page) = separated_list1(nomchar(','), nomu8)(input)?;
    Ok((input, page))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1() {
        let input = fs::read_to_string("./part1-example1.txt").unwrap();
        let result = process_part1(&input);
        assert_eq!(result, "143");
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("./part1-example1.txt").unwrap();
        let result = process_part2(&input);
        assert_eq!(result, "123");
    }
}
