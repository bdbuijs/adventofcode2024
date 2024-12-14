use std::collections::HashMap;

use nom::{
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
    IResult,
};

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub fn process_part1(input: &str) -> String {
    let (input, garden) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let height = garden.len();
    let width = garden[0].len();
    let iwidth = width as isize;
    let iheight = height as isize;
    let mut uf = UnionFind::new(width, height);
    (0..height).for_each(|y| {
        (0..width).for_each(|x| {
            let c = garden[y][x];
            let idx = y * width + x;
            DIRECTIONS.iter().copied().for_each(|(dx, dy)| {
                let nbor_x = x as isize + dx;
                let nbor_y = y as isize + dy;
                if nbor_y >= 0 && nbor_y < iheight && nbor_x >= 0 && nbor_x < iwidth {
                    if c == garden[nbor_y as usize][nbor_x as usize] {
                        let nbor_idx = nbor_y as usize * width + nbor_x as usize;
                        uf.union(idx, nbor_idx);
                    } else {
                        uf.increase_perimeter(x, y);
                    }
                } else {
                    uf.increase_perimeter(x, y);
                }
            });
        });
    });
    uf.check();
    let cost = uf
        .perimeter
        .iter()
        .enumerate()
        .filter_map(|(idx, &perimeter)| {
            if perimeter > 0 {
                let area = uf.parents.iter().filter(|&&p| p == idx).count();
                Some(area * perimeter as usize)
            } else {
                None
            }
        })
        .sum::<usize>();

    cost.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (input, garden) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let height = garden.len();
    let width = garden[0].len();
    let iwidth = width as isize;
    let iheight = height as isize;
    let mut uf = UnionFind::new(width, height);
    (0..height).for_each(|y| {
        (0..width).for_each(|x| {
            let c = garden[y][x];
            let idx = y * width + x;
            DIRECTIONS.iter().copied().for_each(|(dx, dy)| {
                let nbor_x = x as isize + dx;
                let nbor_y = y as isize + dy;
                if nbor_y >= 0 && nbor_y < iheight && nbor_x >= 0 && nbor_x < iwidth {
                    if c == garden[nbor_y as usize][nbor_x as usize] {
                        let nbor_idx = nbor_y as usize * width + nbor_x as usize;
                        uf.union(idx, nbor_idx);
                    } else {
                        uf.add_edge(x, y, dx, dy);
                    }
                } else {
                    uf.add_edge(x, y, dx, dy);
                }
            });
        });
    });
    uf.check();
    uf.union_edges();
    let cost = uf
        .edges
        .iter()
        .map(|(k, v)| {
            assert!(!v.is_empty());
            let area = uf.parents.iter().filter(|&p| p == k).count();
            area * v.len()
        })
        .sum::<usize>();
    cost.to_string()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EdgeType {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Edge {
    start: Point,
    end: Point,
    typ: EdgeType,
}

impl Edge {
    fn empty() -> Self {
        Self {
            start: Point { x: 0, y: 0 },
            end: Point { x: 0, y: 0 },
            typ: EdgeType::Top,
        }
    }

    fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

#[derive(Debug)]
struct UnionFind {
    width: usize,
    height: usize,
    perimeter: Vec<u16>,
    edges: HashMap<usize, Vec<Edge>>,
    parents: Vec<usize>,
    rank: Vec<u8>,
}

impl UnionFind {
    fn new(width: usize, height: usize) -> Self {
        let perimeter = vec![0_u16; width * height];
        let parents = (0_usize..).take(width * height).collect::<Vec<_>>();
        let rank = vec![0_u8; width * height];
        let edges = HashMap::new();
        Self {
            width,
            height,
            perimeter,
            edges,
            parents,
            rank,
        }
    }

    fn find(&mut self, x: usize, y: usize) -> usize {
        let idx = y * self.width + x;
        self.find_by_idx(idx)
    }

    fn find_by_idx(&mut self, idx: usize) -> usize {
        if self.parents[idx] != idx {
            self.parents[idx] = self.find_by_idx(self.parents[idx]);
        }
        self.parents[idx]
    }

    fn union(&mut self, idx1: usize, idx2: usize) {
        let root1 = self.find_by_idx(idx1);
        let root2 = self.find_by_idx(idx2);
        if root1 == root2 {
            return;
        }
        match self.rank[root1].cmp(&self.rank[root2]) {
            std::cmp::Ordering::Less => {
                self.parents[root1] = root2;
                self.perimeter[root2] += self.perimeter[root1];
                self.perimeter[root1] = 0;
                if let Some(v) = self.edges.remove(&root1) {
                    if let Some(v2) = self.edges.get_mut(&root2) {
                        v2.extend(v);
                    } else {
                        self.edges.insert(root2, v);
                    }
                }
            }
            std::cmp::Ordering::Equal => {
                let min = root1.min(root2);
                let max = root1.max(root2);
                self.parents[max] = min;
                self.rank[min] += 1;
                self.perimeter[min] += self.perimeter[max];
                self.perimeter[max] = 0;
                if let Some(v) = self.edges.remove(&max) {
                    if let Some(v2) = self.edges.get_mut(&min) {
                        v2.extend(v);
                    } else {
                        self.edges.insert(min, v);
                    }
                }
            }
            std::cmp::Ordering::Greater => {
                self.parents[root2] = root1;
                self.perimeter[root1] += self.perimeter[root2];
                self.perimeter[root2] = 0;
                if let Some(v) = self.edges.remove(&root2) {
                    if let Some(v2) = self.edges.get_mut(&root1) {
                        v2.extend(v);
                    } else {
                        self.edges.insert(root1, v);
                    }
                }
            }
        }
    }

    fn increase_perimeter(&mut self, x: usize, y: usize) {
        let idx = self.find(x, y);
        self.perimeter[idx] += 1;
    }

    fn add_edge(&mut self, x: usize, y: usize, dx: isize, dy: isize) {
        let idx = y * self.width + x;
        let (start, end, typ) = if dx == -1 {
            (Point::new(x, y), Point::new(x, y + 1), EdgeType::Left)
        } else if dx == 1 {
            (
                Point::new(x + 1, y),
                Point::new(x + 1, y + 1),
                EdgeType::Right,
            )
        } else if dy == -1 {
            (Point::new(x, y), Point::new(x + 1, y), EdgeType::Top)
        } else {
            assert_eq!(dy, 1);
            (
                Point::new(x, y + 1),
                Point::new(x + 1, y + 1),
                EdgeType::Bottom,
            )
        };
        let edge = Edge { start, end, typ };
        self.edges
            .entry(idx)
            .and_modify(|v| {
                v.push(edge.clone());
            })
            .or_insert_with(|| vec![edge]);
    }

    // make sure tree is only 1 layer deep before calculating areas
    fn check(&mut self) {
        (0..(self.width * self.height)).for_each(|idx| {
            self.find_by_idx(idx);
        });
        self.parents.iter().enumerate().for_each(|(i, &p)| {
            if i != p {
                assert_eq!(p, self.parents[p]);
            }
        });
        self.parents.iter().enumerate().for_each(|(idx, &parent)| {
            if parent != idx {
                if let Some(v) = self.edges.remove(&idx) {
                    self.edges
                        .get_mut(&parent)
                        .expect("Invalid edge tree")
                        .extend(v);
                }
            }
        });
    }

    fn union_edges(&mut self) {
        self.edges.iter_mut().for_each(|(_k, v)| {
            (0..v.len()).for_each(|idx| {
                let mut first = v[idx].clone();
                if first.is_empty() {
                    return;
                }
                let mut end = first.end;
                while let Some(second_idx) =
                    v.iter().position(|e| e.start == end && e.typ == first.typ)
                {
                    first.end = v[second_idx].end;
                    end = first.end;
                    v[second_idx] = Edge::empty();
                }
                v[idx] = first;
            });
            v.retain(|e| !e.is_empty());
        });
    }
}

type Line = Vec<char>;

fn parse_input(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, line) = many1(one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ"))(input)?;
    Ok((input, line))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_1() {
        let input = fs::read_to_string("./part1-example1.txt").unwrap();
        let result = process_part1(&input);
        assert_eq!(result, "140");
    }

    #[test]
    fn part1_2() {
        let input = fs::read_to_string("./part1-example2.txt").unwrap();
        let result = process_part1(&input);
        assert_eq!(result, "772");
    }

    #[test]
    fn part1_3() {
        let input = fs::read_to_string("./part1-example3.txt").unwrap();
        let result = process_part1(&input);
        assert_eq!(result, "1930");
    }

    #[test]
    fn part2_0() {
        let input = fs::read_to_string("./part1-example1.txt").unwrap();
        let result = process_part2(&input);
        assert_eq!(result, "80");
    }

    #[test]
    fn part2_1() {
        let input = fs::read_to_string("./part1-example2.txt").unwrap();
        let result = process_part2(&input);
        assert_eq!(result, "436");
    }

    #[test]
    fn part2_2() {
        let input = fs::read_to_string("./part2-example1.txt").unwrap();
        let result = process_part2(&input);
        assert_eq!(result, "236");
    }

    #[test]
    fn part2_3() {
        let input = fs::read_to_string("./part2-example2.txt").unwrap();
        let result = process_part2(&input);
        assert_eq!(result, "368");
    }
}
