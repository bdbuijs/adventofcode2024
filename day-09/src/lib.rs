pub fn process_part1(input: &str) -> String {
    let (mut disk, _, _) = parse_input(input);
    let mut i = 0;
    loop {
        if i >= disk.len() {
            break;
        }
        if disk[i].is_empty() {
            disk.swap_remove(i);
        } else {
            i += 1;
        }
    }
    let checksum = disk
        .iter()
        .enumerate()
        .map(|(i, d)| match d {
            DiskSpace::Empty => panic!("No space should be left on disk at this point"),
            DiskSpace::File(ref id) => id * i,
        })
        .sum::<usize>();
    checksum.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_disk, mut space, mut files) = parse_input(input);
    files.iter_mut().rev().for_each(|f| {
        if let Some(e) = space
            .iter_mut()
            .find(|e| e.len >= f.len && e.start < f.start)
        {
            f.start = e.start;
            e.start += f.len;
            e.len -= f.len;
        }
    });
    let checksum = files
        .into_iter()
        .enumerate()
        .flat_map(|(id, f)| (f.start..).take(f.len).map(move |i| i * id))
        .sum::<usize>();
    checksum.to_string()
}

fn parse_input(input: &str) -> (Vec<DiskSpace>, Vec<EmptySpace>, Vec<FileSpace>) {
    let mut disk = Vec::new();
    let mut empty_space = Vec::new();
    let mut files = Vec::new();
    input.char_indices().for_each(|(i, n)| {
        let n = n.to_digit(10).expect("Invalid digit!") as usize;
        match i % 2 {
            0 => {
                files.push(FileSpace {
                    start: disk.len(),
                    len: n,
                });
                disk.extend(std::iter::repeat_n(DiskSpace::File(i / 2), n))
            }
            1 => {
                empty_space.push(EmptySpace {
                    start: disk.len(),
                    len: n,
                });
                disk.extend(std::iter::repeat_n(DiskSpace::Empty, n))
            }
            _ => unreachable!(),
        }
    });
    assert!(disk.last().expect("Invalid disk!").is_file());
    (disk, empty_space, files)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum DiskSpace {
    Empty,
    File(usize),
}

impl DiskSpace {
    fn is_file(&self) -> bool {
        match self {
            DiskSpace::Empty => false,
            DiskSpace::File(_) => true,
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            DiskSpace::Empty => true,
            DiskSpace::File(_) => false,
        }
    }
}

struct EmptySpace {
    start: usize,
    len: usize,
}

struct FileSpace {
    start: usize,
    len: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1() {
        let input = fs::read_to_string("./part1-example1.txt").unwrap();
        let result = process_part1(&input);
        assert_eq!(result, "1928");
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("./part1-example1.txt").unwrap();
        let result = process_part2(&input);
        assert_eq!(result, "2858");
    }
}
