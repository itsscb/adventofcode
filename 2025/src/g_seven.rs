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

#[allow(clippy::suspicious_map)]
#[must_use]
pub fn solve_seven(input: &[u8]) -> usize {
    let lines: Vec<_> = input.split(|b| b == &b'\n').collect();

    (0..lines[0].len())
        .flat_map(|x| (0..lines.len()).map(move |y| (x, y)))
        .filter(|&(x, y)| {
            if let Some(y1) = lines.get(y) {
                if let Some(x1) = y1.get(x) {
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
            }
            false
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_seven() {
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
        assert_eq!(solve_seven(input), 13);
    }
}
