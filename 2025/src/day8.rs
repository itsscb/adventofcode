use std::collections::HashSet;

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
            .split(',')
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
    (a.x.abs_diff(b.x)).pow(2) + (a.y.abs_diff(b.y)).pow(2) + (a.z.abs_diff(b.z)).pow(2)
}

#[must_use]
pub fn part2(_input: &[u8]) -> usize {
    0
}

pub fn part1(input: &[u8]) -> usize {
    const MAX: i32 = if cfg!(test) { 10 } else { 1000 };

    let input = unsafe { std::str::from_utf8_unchecked(input) };
    let positions: Vec<Position> = input.lines().map(Position::from).collect();

    let mut pairs: Vec<(usize, usize, u64)> = vec![];

    for (i, p1) in positions.iter().enumerate() {
        for (j, p2) in positions.iter().enumerate().filter(|(j, _)| &i != j) {
            let dist = get_distance(p1, p2);
            pairs.push((i, j, dist));
        }
    }

    pairs.sort_by(|a, b| Ord::cmp(&b.2, &a.2));

    let mut circs: Vec<HashSet<usize>> = vec![];
    let mut count = 0;

    for (a, b, _) in pairs {
        if count >= MAX {
            break;
        }
        let index = circs.iter().position(|p| p.contains(&a));
        if let Some(index) = index {
            circs[index].insert(b);
            count += 1;
            continue;
        }
        let index = circs.iter().position(|p| p.contains(&b));
        if let Some(index) = index {
            circs[index].insert(a);
            count += 1;
            continue;
        }

        circs.push(HashSet::from([a, b]));
    }
    dbg!(&circs);
    circs.iter().take(3).map(|h| dbg!(h.len())).product()
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
