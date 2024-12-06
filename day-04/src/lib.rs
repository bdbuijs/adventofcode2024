pub fn process_part1(input: &str) -> String {
    let chars = parse_input(input);
    let width = chars[0].len();
    let height = chars.len();
    let mut string = String::new();
    // left to right
    chars.iter().for_each(|c| {
        string.extend(c.iter().copied());
        string.push(' ');
    });
    // down
    (0..width).for_each(|x| {
        (0..height).for_each(|y| string.push(chars[y][x]));
        string.push(' ');
    });
    // positive diagonal
    (0..height).for_each(|starty| {
        (0..=starty).rev().for_each(|y| {
            let x = starty - y;
            string.push(chars[y][x]);
        });
        string.push(' ');
    });
    (1..width).for_each(|startx| {
        (startx..width).enumerate().for_each(|(i, x)| {
            let y = height - i - 1;
            string.push(chars[y][x]);
        });
        string.push(' ');
    });
    // negative diagonal
    (0..width).rev().for_each(|startx| {
        (startx..width)
            .enumerate()
            .for_each(|(y, x)| string.push(chars[y][x]));
        string.push(' ');
    });
    (1..height).for_each(|starty| {
        (starty..height).enumerate().for_each(|(x, y)| {
            string.push(chars[y][x]);
        });
        string.push(' ');
    });
    let reverse = string.chars().rev().collect::<String>();
    string.push_str(&reverse);

    string.matches("XMAS").count().to_string()
}

pub fn process_part2(input: &str) -> String {
    let chars = parse_input(input);
    let width = chars[0].len();
    let height = chars.len();
    let count = (1..(width - 1))
        .flat_map(|x| (1..(height - 1)).map(move |y| (x, y)))
        .filter(|&(x, y)| chars[y][x] == 'A')
        .filter(|&(x, y)| {
            (chars[y - 1][x - 1] == 'M' && chars[y + 1][x + 1] == 'S'
                || chars[y - 1][x - 1] == 'S' && chars[y + 1][x + 1] == 'M')
                && (chars[y + 1][x - 1] == 'M' && chars[y - 1][x + 1] == 'S'
                    || chars[y + 1][x - 1] == 'S' && chars[y - 1][x + 1] == 'M')
        })
        .count();
    count.to_string()
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut table = Vec::new();
    let mut row = Vec::new();
    for c in input.chars() {
        match c {
            '\n' => {
                table.push(row);
                row = Vec::new();
            }
            x => row.push(x),
        }
    }
    table.push(row);
    table
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1() {
        let input = fs::read_to_string("./part1-example1.txt").unwrap();
        let result = process_part1(&input);
        assert_eq!(result, "18");
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("./part1-example1.txt").unwrap();
        let result = process_part2(&input);
        assert_eq!(result, "9");
    }
}
