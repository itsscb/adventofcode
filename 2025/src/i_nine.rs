use std::collections::HashSet;

pub fn solve_nine(input: &[u8]) -> usize {
    let pos = input
        .windows(2)
        .position(|w| w == b"\n\n")
        .expect("has been promised");
    let (config, ingredients) = (&input[..pos], &input[pos + 2..]);

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
    use crate::i_nine::solve_nine;

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
}
