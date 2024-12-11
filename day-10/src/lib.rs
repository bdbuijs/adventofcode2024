use std::collections::HashSet;

pub fn process_part1(input: &str) -> String {
    let map = parse_input(input);
    let width = map[0].len();
    let height = map.len();
    let mut stack = Vec::new();
    let mut trailhead = HashSet::new();
    let sum = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, h)| (x, y, *h)))
        .filter(|(_x, _y, h)| h == &0)
        .map(|(x, y, h)| {
            stack.push((x, y, h));
            while let Some((x, y, h)) = stack.pop() {
                if h == 9 {
                    trailhead.insert((x, y));
                    continue;
                }
                if x > 0 && map[y][x - 1] == h + 1 {
                    stack.push((x - 1, y, h + 1));
                }
                if x < (width - 1) && map[y][x + 1] == h + 1 {
                    stack.push((x + 1, y, h + 1));
                }
                if y > 0 && map[y - 1][x] == h + 1 {
                    stack.push((x, y - 1, h + 1));
                }
                if y < (height - 1) && map[y + 1][x] == h + 1 {
                    stack.push((x, y + 1, h + 1));
                }
            }
            let score = trailhead.len();
            trailhead.clear();
            score
        })
        .sum::<usize>();
    sum.to_string()
}

pub fn process_part2(input: &str) -> String {
    let map = parse_input(input);
    let width = map[0].len();
    let height = map.len();
    let mut stack = Vec::new();
    let sum = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, h)| (x, y, *h)))
        .filter(|(_x, _y, h)| h == &0)
        .map(|(x, y, h)| {
            stack.push((x, y, h));
            let mut rating = 0;
            while let Some((x, y, h)) = stack.pop() {
                if h == 9 {
                    rating += 1;
                    continue;
                }
                if x > 0 && map[y][x - 1] == h + 1 {
                    stack.push((x - 1, y, h + 1));
                }
                if x < (width - 1) && map[y][x + 1] == h + 1 {
                    stack.push((x + 1, y, h + 1));
                }
                if y > 0 && map[y - 1][x] == h + 1 {
                    stack.push((x, y - 1, h + 1));
                }
                if y < (height - 1) && map[y + 1][x] == h + 1 {
                    stack.push((x, y + 1, h + 1));
                }
            }
            rating
        })
        .sum::<usize>();
    sum.to_string()
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    let mut v = Vec::new();
    input.lines().for_each(|row| {
        v.push(
            row.chars()
                .map(|c| c.to_digit(10).expect("invalid digit in input") as u8)
                .collect(),
        )
    });
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1() {
        let input = fs::read_to_string("./part1-example1.txt").unwrap();
        let result = process_part1(&input);
        assert_eq!(result, "36");
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("./part1-example1.txt").unwrap();
        let result = process_part2(&input);
        assert_eq!(result, "81");
    }
}
