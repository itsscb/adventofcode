#[derive(Debug, Clone, Copy, Hash, Eq, Ord, PartialEq, PartialOrd)]
struct Point {
    row: usize,
    col: usize,
}

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

#[must_use]
pub fn solve_eight(input: &[u8]) -> usize {
    let input = std::str::from_utf8(input).unwrap();
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
    fn test_solve_eight() {
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
        assert_eq!(solve_eight(input), 43);
    }
}
