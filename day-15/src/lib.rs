use std::fmt::Debug;

use nom::{
    bytes::complete::tag,
    character::complete::{newline, one_of},
    error::make_error,
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (input, (mut warehouse, moves)) = parse_input(input).unwrap();
    assert!(input.is_empty());
    warehouse.move_robot(moves);
    warehouse.sum().to_string()
}

pub fn process_part2(input: &str) -> String {
    let new_input = transform_input(input);
    let input = new_input.as_str();
    let (input, (mut warehouse, moves)) = parse_input2(input).unwrap();
    assert!(input.is_empty());
    warehouse.move_robot(moves);
    warehouse.sum().to_string()
}

#[derive(Debug, Clone, Copy)]
enum Space {
    Empty,
    Box,
    Wall,
}

impl From<&Space> for char {
    fn from(val: &Space) -> Self {
        match val {
            Space::Empty => '.',
            Space::Box => 'O',
            Space::Wall => '#',
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Space2 {
    Empty,
    LeftBox,
    RightBox,
    Wall,
}

impl From<&Space2> for char {
    fn from(val: &Space2) -> Self {
        match val {
            Space2::Empty => '.',
            Space2::Wall => '#',
            Space2::LeftBox => '[',
            Space2::RightBox => ']',
        }
    }
}

struct Robot {
    x: usize,
    y: usize,
}

struct Warehouse {
    contents: Vec<Vec<Space>>,
    robot: Robot,
}

impl Warehouse {
    fn move_robot(&mut self, moves: Vec<RobotMove>) {
        for robotmove in moves.into_iter() {
            let (new_x, new_y) = Self::next_pos(self.robot.x, self.robot.y, robotmove);
            match self.contents[new_y][new_x] {
                Space::Empty => {
                    (self.robot.x, self.robot.y) = (new_x, new_y);
                }
                Space::Box => {
                    if self.move_box(new_x, new_y, robotmove) {
                        (self.robot.x, self.robot.y) = (new_x, new_y)
                    }
                }
                Space::Wall => continue,
            }
        }
    }

    fn move_box(&mut self, x: usize, y: usize, robotmove: RobotMove) -> bool {
        assert!(y < self.contents.len());
        assert!(x < self.contents[0].len());
        assert!(matches!(self.contents[y][x], Space::Box));
        let (next_x, next_y) = Self::next_pos(x, y, robotmove);
        match self.contents[next_y][next_x] {
            Space::Empty => {
                self.contents[next_y][next_x] = Space::Box;
                self.contents[y][x] = Space::Empty;
                true
            }
            Space::Box => {
                if self.move_box(next_x, next_y, robotmove) {
                    self.contents[next_y][next_x] = Space::Box;
                    self.contents[y][x] = Space::Empty;
                    true
                } else {
                    false
                }
            }
            Space::Wall => false,
        }
    }

    fn next_pos(x: usize, y: usize, robotmove: RobotMove) -> (usize, usize) {
        match robotmove {
            RobotMove::Up => (x, y - 1),
            RobotMove::Down => (x, y + 1),
            RobotMove::Left => (x - 1, y),
            RobotMove::Right => (x + 1, y),
        }
    }
    fn sum(&self) -> usize {
        self.contents
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(x, space)| match space {
                        Space::Box => Some(100 * y + x),
                        _ => None,
                    })
            })
            .sum()
    }
}

impl Debug for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        self.contents.iter().enumerate().for_each(|(y, row)| {
            if y == self.robot.y {
                row.iter().enumerate().for_each(|(x, space)| {
                    if x == self.robot.x {
                        s.push('@');
                    } else {
                        s.push(space.into());
                    }
                });
            } else {
                row.iter().for_each(|space| s.push(space.into()));
            }
            s.push('\n');
        });

        f.write_str(&s)
    }
}

struct Warehouse2 {
    contents: Vec<Vec<Space2>>,
    robot: Robot,
}

impl Warehouse2 {
    fn move_robot(&mut self, moves: Vec<RobotMove>) {
        for robotmove in moves.into_iter() {
            let (new_x, new_y) = Self::next_pos(self.robot.x, self.robot.y, robotmove);
            match self.contents[new_y][new_x] {
                Space2::Empty => {
                    (self.robot.x, self.robot.y) = (new_x, new_y);
                }
                Space2::LeftBox | Space2::RightBox => {
                    if self.move_box(new_x, new_y, robotmove, true) {
                        self.move_box(new_x, new_y, robotmove, false);
                        (self.robot.x, self.robot.y) = (new_x, new_y)
                    }
                }
                Space2::Wall => continue,
            }
        }
    }

    fn move_box(&mut self, x: usize, y: usize, robotmove: RobotMove, dry_run: bool) -> bool {
        assert!(y < self.contents.len());
        assert!(x < self.contents[0].len());
        match self.contents[y][x] {
            Space2::LeftBox => {
                if !self.move_other_box(x + 1, y, robotmove, dry_run) {
                    return false;
                }
                if matches!(robotmove, RobotMove::Right) {
                    if !dry_run {
                        self.contents[y][x + 1] = Space2::LeftBox;
                        self.contents[y][x] = Space2::Empty;
                    }
                    return true;
                }
            }
            Space2::RightBox => {
                if !self.move_other_box(x - 1, y, robotmove, dry_run) {
                    return false;
                }
                if matches!(robotmove, RobotMove::Left) {
                    if !dry_run {
                        self.contents[y][x - 1] = Space2::RightBox;
                        self.contents[y][x] = Space2::Empty;
                    }
                    return true;
                }
            }
            x => panic!("move_box called on {x:?}"),
        }
        let (next_x, next_y) = Self::next_pos(x, y, robotmove);
        match self.contents[next_y][next_x] {
            Space2::Empty => {
                if !dry_run {
                    self.contents[next_y][next_x] = self.contents[y][x];
                    self.contents[y][x] = Space2::Empty;
                }
                true
            }
            Space2::LeftBox | Space2::RightBox => {
                if self.move_box(next_x, next_y, robotmove, dry_run) {
                    if !dry_run {
                        self.contents[next_y][next_x] = self.contents[y][x];
                        self.contents[y][x] = Space2::Empty;
                    }
                    true
                } else {
                    false
                }
            }
            Space2::Wall => false,
        }
    }

    fn move_other_box(&mut self, x: usize, y: usize, robotmove: RobotMove, dry_run: bool) -> bool {
        assert!(y < self.contents.len());
        assert!(x < self.contents[0].len());
        assert!(matches!(
            self.contents[y][x],
            Space2::LeftBox | Space2::RightBox
        ));
        let (next_x, next_y) = Self::next_pos(x, y, robotmove);
        match self.contents[next_y][next_x] {
            Space2::Empty => {
                if !dry_run {
                    self.contents[next_y][next_x] = self.contents[y][x];
                    self.contents[y][x] = Space2::Empty;
                }
                true
            }
            Space2::LeftBox | Space2::RightBox => {
                if self.move_box(next_x, next_y, robotmove, dry_run) {
                    if !dry_run {
                        self.contents[next_y][next_x] = self.contents[y][x];
                        self.contents[y][x] = Space2::Empty;
                    }
                    true
                } else {
                    false
                }
            }
            Space2::Wall => false,
        }
    }

    fn next_pos(x: usize, y: usize, robotmove: RobotMove) -> (usize, usize) {
        match robotmove {
            RobotMove::Up => (x, y - 1),
            RobotMove::Down => (x, y + 1),
            RobotMove::Left => (x - 1, y),
            RobotMove::Right => (x + 1, y),
        }
    }

    fn sum(&self) -> usize {
        self.contents
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(x, space)| match space {
                        Space2::LeftBox => Some(100 * y + x),
                        _ => None,
                    })
            })
            .sum()
    }
}

#[derive(Debug, Clone, Copy)]
enum RobotMove {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for RobotMove {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::Up,
            'v' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            c => panic!("Invalid character for RobotMove: {c}"),
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, (Warehouse, Vec<RobotMove>)> {
    let (input, warehouse) = parse_warehouse(input)?;
    let (input, moves) = parse_moves(input)?;
    Ok((input, (warehouse, moves)))
}

fn parse_input2(input: &str) -> IResult<&str, (Warehouse2, Vec<RobotMove>)> {
    let (input, warehouse) = parse_warehouse2(input)?;
    let (input, moves) = parse_moves(input)?;
    Ok((input, (warehouse, moves)))
}

fn parse_warehouse(input: &str) -> IResult<&str, Warehouse> {
    let (input, warehouse_chars) =
        terminated(separated_list1(newline, many1(one_of(".O#@"))), tag("\n\n"))(input)?;
    let mut robot: Option<Robot> = None;
    let mut contents = Vec::new();
    warehouse_chars
        .into_iter()
        .enumerate()
        .for_each(|(y, row)| {
            let mut warehouse_row = Vec::new();
            row.into_iter().enumerate().for_each(|(x, c)| match c {
                '.' => warehouse_row.push(Space::Empty),
                'O' => warehouse_row.push(Space::Box),
                '#' => warehouse_row.push(Space::Wall),
                '@' => {
                    warehouse_row.push(Space::Empty);
                    robot.replace(Robot { x, y });
                }
                other => panic!("Unexpecter character in warehouse: {other}"),
            });
            contents.push(warehouse_row);
        });
    let robot = robot.expect("Robot missing!");
    let warehouse = Warehouse { contents, robot };

    Ok((input, warehouse))
}

fn parse_warehouse2(input: &str) -> IResult<&str, Warehouse2> {
    let (input, warehouse_chars) = terminated(
        separated_list1(newline, many1(one_of(".[]#@"))),
        tag("\n\n"),
    )(input)?;
    let mut robot: Option<Robot> = None;
    let mut contents = Vec::new();
    warehouse_chars
        .into_iter()
        .enumerate()
        .for_each(|(y, row)| {
            let mut warehouse_row = Vec::new();
            row.into_iter().enumerate().for_each(|(x, c)| match c {
                '.' => warehouse_row.push(Space2::Empty),
                '[' => warehouse_row.push(Space2::LeftBox),
                ']' => warehouse_row.push(Space2::RightBox),
                '#' => warehouse_row.push(Space2::Wall),
                '@' => {
                    warehouse_row.push(Space2::Empty);
                    robot.replace(Robot { x, y });
                }
                other => panic!("Unexpecter character in warehouse: {other}"),
            });
            contents.push(warehouse_row);
        });
    let robot = robot.expect("Robot missing!");
    let warehouse = Warehouse2 { contents, robot };

    Ok((input, warehouse))
}

fn parse_moves(input: &str) -> IResult<&str, Vec<RobotMove>> {
    let mut moves = Vec::new();
    for c in input.chars() {
        match c {
            '\n' => continue,
            '^' | 'v' | '<' | '>' => {
                moves.push(c.into());
            }
            _ => {
                return Err(nom::Err::Error(make_error(
                    input,
                    nom::error::ErrorKind::Fail,
                )))
            }
        }
    }
    Ok(("", moves))
}

fn transform_input(input: &str) -> String {
    let mut output = String::new();
    input.chars().for_each(|c| {
        let out = match c {
            '.' => "..",
            '#' => "##",
            '@' => "@.",
            'O' => "[]",
            '\n' => "\n",
            '^' => "^",
            'v' => "v",
            '<' => "<",
            '>' => ">",
            x => panic!("Unexpected character in input: {x}"),
        };
        output.push_str(out);
    });

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_1() {
        let input = fs::read_to_string("./part1-example1.txt").unwrap();
        let result = process_part1(&input);
        assert_eq!(result, "2028");
    }

    #[test]
    fn part1_2() {
        let input = fs::read_to_string("./part1-example2.txt").unwrap();
        let result = process_part1(&input);
        assert_eq!(result, "10092");
    }

    #[test]
    fn part2_1() {
        let input = fs::read_to_string("./part2-example1.txt").unwrap();
        let result = process_part2(&input);
        assert_eq!(result, "618");
    }

    #[test]
    fn part2_2() {
        let input = fs::read_to_string("./part1-example2.txt").unwrap();
        let result = process_part2(&input);
        assert_eq!(result, "9021");
    }
}
