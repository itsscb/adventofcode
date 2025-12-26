use std::{collections::HashSet, u64};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default, Eq, Ord, Hash)]
struct Position {
    x: u64,
    y: u64,
    z: u64,
}

impl<T: AsRef<str>> From<T> for Position {
    fn from(value: T) -> Self {
        let mut s: Self = Self::default();
        value
            .as_ref()
            .split(|c| c == ',')
            .enumerate()
            .take(3)
            .map(|(i, v)| (i, v.parse::<u64>().expect("was promised")))
            .for_each(|(i, v)| match i {
                0 => s.x = v,
                1 => s.y = v,
                2 => s.z = v,
                _ => unreachable!("was promised"),
            });
        s
    }
}

#[must_use]
#[allow(dead_code)]
fn get_distance(a: &Position, b: &Position) -> u64 {
    ((a.x - b.x) ^ 2 + (a.y - b.y) ^ 2 + (a.z - b.z) ^ 2).isqrt()
}

fn get_next<'a>(positions: &'a Vec<Position>, current: &'a Position) -> &'a Position {
    let mut min_distance = u64::MAX;
    let mut pos = 0usize;
    positions
        .iter()
        .map(|p| get_distance(current, p))
        .enumerate()
        .for_each(|(i, p)| {
            if p < min_distance {
                min_distance = p;
                pos = i;
            }
        });

    &positions[pos]
}

pub fn part1(input: &[u8]) -> usize {
    let input = unsafe { std::str::from_utf8_unchecked(input) };
    let mut circuits: Vec<HashSet<Position>> = vec![HashSet::with_capacity(10); 1000];
    let positions: Vec<Position> = input.lines().map(Position::from).collect();

    positions
        .iter()
        .map(|p| (p, get_next(&positions, p)))
        .enumerate()
        .for_each(|(i, (a, b))| {
            circuits[i].insert(a.clone());
            circuits[i].insert(b.clone());
        });
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = b"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

        assert_eq!(40, part1(input));
    }
}
