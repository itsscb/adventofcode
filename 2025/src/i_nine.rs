use std::collections::HashSet;

pub fn solve_nine(input: &[u8]) -> usize {
    let (config, ingredients) = split_input(input);

    let minmax: HashSet<(u64, u64)> = config.split(|b| b == &b'\n').map(get_min_max).collect();

    ingredients
        .split(|b| b == &b'\n')
        .map(std::str::from_utf8)
        .flatten()
        .map(|i| i.parse())
        .flatten()
        .filter(|i: &u64| minmax.iter().any(|(min, max)| i >= &min && i <= &max))
        .count()
}

pub fn solve_ten(input: &[u8]) -> u64 {
    let (config, _) = split_input(input);

    let mut ranges: Vec<(u64, u64)> = config.split(|b| b == &b'\n').map(get_min_max).collect();
    ranges.sort_unstable_by_key(|k| k.0);

    let mut count = 0;

    let mut range = ranges.into_iter();
    let (mut curr_min, mut curr_max) = range.next().expect("first always given");

    while let Some((min, max)) = range.next() {
        if curr_max < min {
            count += curr_max - curr_min + 1;
            (curr_min, curr_max) = (min, max);
        } else {
            curr_max = curr_max.max(max);
        }
    }

    count += curr_max - curr_min + 1;
    count
}

fn split_input(input: &[u8]) -> (&[u8], &[u8]) {
    let pos = input
        .windows(2)
        .position(|w| w == b"\n\n")
        .expect("has been promised");
    (&input[..pos], &input[pos + 2..])
}

fn get_min_max(range: &[u8]) -> (u64, u64) {
    let pos = range
        .iter()
        .position(|b| b == &b'-')
        .expect("has been promised");

    let lhs: u64 = unsafe { std::str::from_utf8_unchecked(&range[..pos]) }
        .parse()
        .unwrap();
    let rhs: u64 = unsafe { std::str::from_utf8_unchecked(&range[pos + 1..]) }
        .parse()
        .unwrap();

    (lhs, rhs)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve_nine() {
        let input = b"3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!(solve_nine(input), 3);
    }
    #[test]
    fn test_solve_ten() {
        let input = b"3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!(solve_ten(input), 14);
    }
}
