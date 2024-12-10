use std::collections::HashSet;

use indicatif::{ProgressBar, ProgressStyle};

use nom::{
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (input, (lab, mut guard)) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let width = lab[0].len();
    let height = lab.len();
    let mut visited = vec![vec![false; width]; height];
    loop {
        visited[guard.y][guard.x] = true;
        let (next_x, next_y) = match guard.direction {
            Direction::Up => {
                if guard.y > 0 {
                    (guard.x, guard.y - 1)
                } else {
                    break;
                }
            }
            Direction::Down => {
                if guard.y < (height - 1) {
                    (guard.x, guard.y + 1)
                } else {
                    break;
                }
            }
            Direction::Left => {
                if guard.x > 0 {
                    (guard.x - 1, guard.y)
                } else {
                    break;
                }
            }
            Direction::Right => {
                if guard.x < (width - 1) {
                    (guard.x + 1, guard.y)
                } else {
                    break;
                }
            }
        };
        match lab[next_y][next_x] {
            Location::Path => {
                guard.set_pos(next_x, next_y);
            }
            Location::Obstruction => guard.turn(),
        }
    }
    let count = visited.into_iter().flatten().filter(|v| *v).count();
    count.to_string()
}

pub fn process_part2(input: &str) -> String {
    let path_count = input.chars().filter(|&c| c == '.').count();
    let (input, (original_lab, original_guard)) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let width = original_lab[0].len();
    let height = original_lab.len();

    let bar = ProgressBar::new(path_count as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:50}] {pos}/{len}")
            .unwrap()
            .progress_chars("=> "),
    );

    let hypothetical_obstructions = (0..height)
        .flat_map(|y| (0..width).map(move |x| (x, y)))
        .filter(|&(x, y)| original_lab[y][x] == Location::Path)
        .filter(|&(x, y)| original_guard.x != x || original_guard.y != y)
        .filter(|&(x, y)| {
            bar.inc(1);
            let mut lab = original_lab.clone();
            lab[y][x] = Location::Obstruction;
            let mut visited = HashSet::new();
            let mut guard = original_guard.clone();
            loop {
                if visited.contains(&guard) {
                    break true;
                }
                visited.insert(guard.clone());
                let (next_x, next_y) = match guard.direction {
                    Direction::Up => {
                        if guard.y > 0 {
                            (guard.x, guard.y - 1)
                        } else {
                            break false;
                        }
                    }
                    Direction::Down => {
                        if guard.y < (height - 1) {
                            (guard.x, guard.y + 1)
                        } else {
                            break false;
                        }
                    }
                    Direction::Left => {
                        if guard.x > 0 {
                            (guard.x - 1, guard.y)
                        } else {
                            break false;
                        }
                    }
                    Direction::Right => {
                        if guard.x < (width - 1) {
                            (guard.x + 1, guard.y)
                        } else {
                            break false;
                        }
                    }
                };
                match lab[next_y][next_x] {
                    Location::Path => {
                        guard.set_pos(next_x, next_y);
                    }
                    Location::Obstruction => guard.turn(),
                }
            }
        })
        .count();
    bar.finish_with_message("Done!");
    hypothetical_obstructions.to_string()
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Location {
    Path,
    Obstruction,
}

impl Location {
    fn parse(input: &str) -> IResult<&str, Self> {
        match one_of(".#^")(input)? {
            (input, '.') | (input, '^') => Ok((input, Self::Path)),
            (input, '#') => Ok((input, Self::Obstruction)),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Guard {
    x: usize,
    y: usize,
    direction: Direction,
}

impl Guard {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            direction: Direction::Up,
        }
    }

    fn set_pos(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    fn turn(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

type Lab = Vec<Vec<Location>>;

fn parse_input(input: &str) -> IResult<&str, (Lab, Guard)> {
    let guard = parse_guard(input);
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, (lines, guard)))
}

fn parse_line(input: &str) -> IResult<&str, Vec<Location>> {
    let (input, row) = many1(Location::parse)(input)?;
    Ok((input, row))
}

fn parse_guard(input: &str) -> Guard {
    let byte_pos = input.find('^').expect("Cannot find guard in input!");
    let line_length = input.find("\n").expect("Only one row in lab!");
    let y = input[..byte_pos].chars().filter(|&c| c == '\n').count();
    let x = (byte_pos - y) % line_length;
    Guard::new(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1() {
        let input = fs::read_to_string("./part1-example1.txt").unwrap();
        let result = process_part1(&input);
        assert_eq!(result, "41");
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("./part1-example1.txt").unwrap();
        let result = process_part2(&input);
        assert_eq!(result, "6");
    }
}
