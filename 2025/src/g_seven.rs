#[allow(clippy::suspicious_map)]
#[must_use]
pub fn solve_seven(input: &[u8]) -> usize {
    // SAFETY: input is valid UTF-8
    let input = unsafe { std::str::from_utf8_unchecked(input) };
    input
        .lines()
        .map(|line| {
            line.chars()
                .enumerate()
                .filter(|(i, c)| {
                    let skip = i.saturating_sub(5);
                    let take = if skip <= 5 { 8 - skip } else { 8 };
                    let m = line
                        .chars()
                        .skip(skip)
                        .take(take)
                        .filter(|c| *c == '@')
                        .count()
                        .saturating_sub(1);

                    println!("{i} {c} {m}");
                    *c == '@' && m < 4
                })
                .count()
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
