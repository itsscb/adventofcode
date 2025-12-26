use std::{collections::HashSet, sync::Mutex};

pub fn solve_thirteen(input: &[u8]) -> usize {
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

    while let Some(line) = lines.next() {
        let mut beams: HashSet<usize> = HashSet::with_capacity(10);
        line.chars()
            .enumerate()
            .filter(|(i, c)| {
                let lock = active_beams.lock().unwrap();
                &*c == &'^' && lock.contains(i)
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
        assert_eq!(solve_thirteen(input), 21);
    }
}
