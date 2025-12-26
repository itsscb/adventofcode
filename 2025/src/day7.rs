#![allow(clippy::missing_panics_doc)]
use std::{collections::HashSet, sync::Mutex};

#[must_use]
pub fn part1(input: &[u8]) -> usize {
    let input = unsafe { std::str::from_utf8_unchecked(input) };
    let active_beams: Mutex<HashSet<usize>> = Mutex::new(HashSet::new());
    let mut count = 0usize;

    let mut lines = input.lines();
    active_beams.lock().unwrap().insert(
        lines
            .next()
            .unwrap()
            .chars()
            .position(|c| c == 'S')
            .expect("has been promised"),
    );

    for line in lines {
        let mut beams: HashSet<usize> = HashSet::with_capacity(10);
        line.chars()
            .enumerate()
            .filter(|(i, c)| {
                let lock = active_beams.lock().unwrap();
                *c == '^' && lock.contains(i)
            })
            .for_each(|(i, _)| {
                count += 1;
                beams.insert(i.saturating_sub(1));
                beams.insert(i.saturating_add(1));
                let mut lock = active_beams.lock().unwrap();
                lock.remove(&i);
            });
        let mut lock = active_beams.lock().unwrap();
        lock.extend(beams);
    }
    count
}

#[must_use]
pub fn part2(input: &[u8]) -> usize {
    let mut lines = input
        .split(|&b| b == b'\n')
        .filter(|line| !line.iter().all(|&b| b == b'.'));
    let first = lines.next().unwrap();

    let mut timelines: Vec<usize> = vec![0; first.len() - 1];
    timelines.insert(first.iter().position(|&b| b == b'S').unwrap(), 1);

    for line in lines {
        timelines
            .clone()
            .iter()
            .enumerate()
            .filter(|(i, _)| line[*i] == b'^')
            .for_each(|(i, count)| {
                timelines[i] = 0;
                timelines[i - 1] += *count;
                timelines[i + 1] += *count;
            });
    }
    timelines.iter().sum::<usize>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve_thriteen() {
        let input = b".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(part1(input), 21);
        assert_eq!(part2(input), 40);
    }
}
