use nom::{
    bytes::complete::tag,
    character::complete::{char as nomchar, digit1, newline, u32 as nomu32},
    combinator::opt,
    multi::separated_list1,
    sequence::terminated,
    IResult,
};
use std::cmp::Ordering;

pub fn process_part1(input: &str) -> String {
    let (input, ((width, height), mut robots)) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let mut quadrants: [u32; 5] = [0; 5];
    robots.iter_mut().for_each(|r| {
        r.walk(width, height, 100);
        let q = r.quadrant(width, height);
        quadrants[q] += 1;
    });
    let safety_factor: u32 = quadrants.into_iter().skip(1).product();
    safety_factor.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (input, ((width, height), mut robots)) = parse_input(input).unwrap();
    assert!(input.is_empty());

    let mut christmas_egg = 0;

    for n in 100..20000 {
        if n % 101 == 13 && n % 103 == 79 {
            christmas_egg = n;
            break;
        }
    }

    let start = christmas_egg;
    robots.iter_mut().for_each(|r| {
        r.walk(width, height, start - 1);
    });
    (start..(start + 1)).for_each(|i| {
        robots.iter_mut().for_each(|r| {
            r.walk(width, height, 1);
        });
        println!("{i}:");
        print_robots(&robots, width, height);
        println!("________________________________________")
    });
    christmas_egg.to_string()
}

#[derive(Debug)]
struct Robot {
    pos: (u32, u32),
    vel: (i32, i32),
}

impl Robot {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("p=")(input)?;
        let (input, px) = nomu32(input)?;
        let (input, _) = nomchar(',')(input)?;
        let (input, py) = nomu32(input)?;
        let pos = (px, py);
        let (input, _) = tag(" v=")(input)?;
        let (input, vx) = parse_i32(input)?;
        let (input, _) = nomchar(',')(input)?;
        let (input, vy) = parse_i32(input)?;
        let vel = (vx, vy);
        Ok((input, Self { pos, vel }))
    }

    fn walk(&mut self, width: u32, height: u32, steps: usize) {
        let (vx, vy) = self.vel;
        let w = width as i32;
        let h = height as i32;
        let s = steps as i32;
        let vx = (vx + w) % w;
        let vy = (vy + h) % h;
        let (mut px, mut py) = self.pos;
        px = ((px as i32 + vx * s) % w) as u32;
        py = ((py as i32 + vy * s) % h) as u32;
        self.pos = (px, py);
    }

    fn quadrant(&self, width: u32, height: u32) -> usize {
        assert!(self.pos.0 < width);
        assert!(self.pos.1 < height);
        let xcenter = width / 2;
        let ycenter = height / 2;
        match (self.pos.0.cmp(&xcenter), self.pos.1.cmp(&ycenter)) {
            (Ordering::Equal, _) | (_, Ordering::Equal) => 0,
            (Ordering::Less, Ordering::Less) => 1,
            (Ordering::Greater, Ordering::Less) => 2,
            (Ordering::Less, Ordering::Greater) => 3,
            (Ordering::Greater, Ordering::Greater) => 4,
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, ((u32, u32), Vec<Robot>)> {
    let (input, width) = terminated(nomu32, nomchar('x'))(input)?;
    let (input, height) = terminated(nomu32, newline)(input)?;

    let (input, robots) = separated_list1(newline, Robot::parse)(input)?;
    Ok((input, ((width, height), robots)))
}

fn parse_i32(input: &str) -> IResult<&str, i32> {
    let (input, sign) = opt(nomchar('-'))(input)?;
    let (input, n) = digit1(input)?;
    let mut i = n.parse::<i32>().expect("Invalid digits");
    if sign.is_some() {
        i = -i;
    }

    Ok((input, i))
}

fn print_robots(robots: &[Robot], width: u32, height: u32) {
    let len = (width * height) as usize;
    let mut string = String::with_capacity(len);
    string.extend(std::iter::repeat('.').take(len));
    robots.iter().for_each(|r| {
        let (x, y) = r.pos;
        let strpos = (y * width + x) as usize;
        assert!(strpos < string.len());
        unsafe {
            let bytes = string.as_bytes_mut();
            bytes[strpos] = b'X';
        }
    });
    for chunk in string.as_bytes().chunks(width as usize) {
        println!("{}", std::str::from_utf8(chunk).unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1() {
        let input = fs::read_to_string("./part1-example1.txt").unwrap();
        let result = process_part1(&input);
        assert_eq!(result, "12");
    }
}
