const MATCHES: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
#[derive(Debug, Clone, Copy, Hash, Eq, Ord, PartialEq, PartialOrd)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn neighbours(&self) -> Vec<Point> {
        MATCHES
            .iter()
            .filter_map(|&(dr, dc)| {
                Some(Point {
                    row: self.row.checked_add_signed(dr)?,
                    col: self.col.checked_add_signed(dc)?,
                })
            })
            .collect()
    }

    fn count_neighbours(&self, map: &[Vec<char>]) -> usize {
        self.neighbours()
            .iter()
            .filter(|point| {
                matches!(
                    map.get(point.row).and_then(|row| row.get(point.col)),
                    Some('@')
                )
            })
            .count()
    }
}

#[allow(clippy::suspicious_map)]
#[must_use]
pub fn part1(input: &[u8]) -> usize {
    let lines: Vec<_> = input.split(|b| b == &b'\n').collect();

    (0..lines[0].len())
        .flat_map(|x| (0..lines.len()).map(move |y| (x, y)))
        .filter(|&(x, y)| {
            if let Some(y1) = lines.get(y)
                && let Some(x1) = y1.get(x)
            {
                return x1 == &b'@'
                    && MATCHES
                        .iter()
                        .filter(|&(x2, y2)| {
                            lines
                                .get(y.wrapping_add_signed(*y2))
                                .and_then(|r| r.get(x.wrapping_add_signed(*x2)))
                                .is_some_and(|&c| c == b'@')
                        })
                        .count()
                        < 4;
            }
            false
        })
        .count()
}

#[must_use]
pub fn part2(input: &[u8]) -> usize {
    let input = unsafe { std::str::from_utf8_unchecked(input) };
    part_2(input)
}

fn part_2(input: &str) -> usize {
    let mut map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut sum = 0;
    loop {
        let mut to_remove = Vec::new();
        for (row, line) in map.iter().enumerate() {
            for (col, c) in line.iter().enumerate() {
                if *c != '@' {
                    continue;
                }
                let point = Point { row, col };
                if point.count_neighbours(&map) < 4 {
                    to_remove.push(point);
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }
        sum += to_remove.len();
        for point in to_remove {
            map[point.row][point.col] = '.';
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = b"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(part2(input), 43);
    }

    #[test]
    fn test_part1() {
        let input = b"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(part1(input), 13);
    }
}
