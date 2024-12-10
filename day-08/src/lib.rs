use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn process_part1(input: &str) -> String {
    let width = input.lines().next().expect("Invalid input!").len() as u8;
    let height = input.lines().count() as u8;
    let map = parse_input(input);
    let anti_nodes = map
        .into_iter()
        .flat_map(|(_k, v)| {
            v.into_iter().combinations(2).flat_map(|c| {
                let ((mut x_a, mut y_a), (mut x_b, mut y_b)) = (c[0], c[1]);
                // make a be the leftmost point:
                if x_a > x_b {
                    std::mem::swap(&mut x_a, &mut x_b);
                    std::mem::swap(&mut y_a, &mut y_b);
                }
                let dx = x_a.abs_diff(x_b);
                let dy = y_a.abs_diff(y_b);
                [
                    x_a.min(x_b).checked_sub(dx).and_then(|left_x| {
                        if y_a < y_b {
                            y_a.checked_sub(dy).map(|left_y| (left_x, left_y))
                        } else {
                            Some((left_x, y_a + dy))
                        }
                    }),
                    {
                        let right_x = x_a.max(x_b) + dx;
                        if y_a < y_b {
                            Some((right_x, y_b + dy))
                        } else {
                            y_b.checked_sub(dy).map(|right_y| (right_x, right_y))
                        }
                    },
                ]
                .into_iter()
                .flatten()
            })
        })
        .filter(|&(x, y)| x < width && y < height)
        .collect::<HashSet<_>>();
    anti_nodes.len().to_string()
}

pub fn process_part2(input: &str) -> String {
    let width = input.lines().next().expect("Invalid input!").len() as u8;
    let height = input.lines().count() as u8;
    let map = parse_input(input);
    let anti_nodes =
        map.into_iter()
            .flat_map(|(_k, v)| {
                v.into_iter().combinations(2).flat_map(|c| {
                    let ((mut x_a, mut y_a), (mut x_b, mut y_b)) = (c[0], c[1]);
                    // make a be the leftmost point:
                    if x_a > x_b {
                        std::mem::swap(&mut x_a, &mut x_b);
                        std::mem::swap(&mut y_a, &mut y_b);
                    }
                    let dx = x_a.abs_diff(x_b);
                    let dy = y_a.abs_diff(y_b);
                    let start_x = x_a % dx;
                    let iter: Box<dyn Iterator<Item = (u8, u8)>> = if dx == 0 {
                        let start_y = y_a.min(y_b) % dy;
                        Box::new(
                            (start_y..)
                                .step_by(dy as usize)
                                .take_while(|y| y < &height)
                                .map(move |y| (x_a, y)),
                        )
                    } else {
                        match y_a.cmp(&y_b) {
                            std::cmp::Ordering::Less => Box::new(
                                (x_b..)
                                    .step_by(dx as usize)
                                    .take_while(|x| x < &width)
                                    .zip((y_b..).step_by(dy as usize).take_while(|y| y < &height))
                                    .chain((0u8..).map_while(move |m| {
                                        y_a.checked_sub(m * dy)
                                            .and_then(|y| x_a.checked_sub(m * dx).map(|x| (x, y)))
                                    })),
                            ),
                            std::cmp::Ordering::Equal => Box::new(
                                (start_x..)
                                    .step_by(dx as usize)
                                    .take_while(|x| x < &width)
                                    .map(move |x| (x, y_a)),
                            ),
                            std::cmp::Ordering::Greater => Box::new(
                                (x_b..)
                                    .step_by(dx as usize)
                                    .take_while(|x| x < &width)
                                    .zip((0..).map_while(move |m| y_b.checked_sub(m * dy)))
                                    .chain((0..).map_while(move |m| x_a.checked_sub(m * dx)).zip(
                                        (y_a..).step_by(dy as usize).take_while(|y| y < &height),
                                    )),
                            ),
                        }
                    };
                    iter
                })
            })
            .collect::<HashSet<_>>();
    anti_nodes.len().to_string()
}

fn parse_input(input: &str) -> HashMap<char, Vec<(u8, u8)>> {
    let mut y = 0;
    let mut map = HashMap::new();
    input.lines().for_each(|line| {
        line.char_indices()
            .filter(|(_, c)| c != &'.')
            .for_each(|(x, c)| {
                map.entry(c)
                    .and_modify(|v: &mut Vec<(u8, u8)>| {
                        v.push((x as u8, y));
                    })
                    .or_insert_with(|| vec![(x as u8, y)]);
            });
        y += 1;
    });
    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1() {
        let input = fs::read_to_string("./part1-example1.txt").unwrap();
        let result = process_part1(&input);
        assert_eq!(result, "14");
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("./part2-example1.txt").unwrap();
        let result = process_part2(&input);
        assert_eq!(result, "9");
    }

    #[test]
    fn part2_2() {
        let input = fs::read_to_string("./part1-example1.txt").unwrap();
        let result = process_part2(&input);
        assert_eq!(result, "34");
    }
}
