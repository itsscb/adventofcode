#[derive(Debug, Clone, Default)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn find_invalid(&self) -> Vec<u64> {
        (self.start..self.end)
            .filter(|&i| {
                let s = format!("{i}");
                let first = &s[0..s.len() / 2];
                let second = &s[s.len() / 2..];
                first == second
            })
            .collect()
    }

    fn find_invalid_thorough(&self) -> Vec<u64> {
        (self.start..self.end)
            .filter(|&i| {
                let s = format!("{i}");
                check_id(&s)
            })
            .collect()
    }
}

#[allow(clippy::unwrap_used)]
fn check_id(id: &str) -> bool {
    let mut window = 1;
    while window < id.len() {
        let mut chunks = id.as_bytes().chunks_exact(window);
        let first = chunks.next().unwrap();

        if chunks.all(|c| c == first) && chunks.remainder().is_empty() {
            return true;
        }

        window += 1;
    }
    false
}

#[allow(clippy::fallible_impl_from, clippy::unwrap_used)]
impl<T: AsRef<[u8]>> From<T> for Range {
    fn from(value: T) -> Self {
        let v: Vec<_> = value
            .as_ref()
            .split(|c| *c == b'-')
            // SAFETY: We know that the input is valid UTF-8.
            .map(|s| unsafe { std::str::from_utf8_unchecked(s) })
            .collect();
        let start: u64 = v[0].parse().unwrap();
        let end: u64 = v[1].parse().unwrap();
        Self { start, end }
    }
}

#[derive(Debug, Clone, Default)]
pub(super) struct Ranges {
    ranges: Vec<Range>,
}

impl Ranges {
    fn find_invalid_ids(&self) -> Vec<u64> {
        self.ranges.iter().flat_map(Range::find_invalid).collect()
    }
    fn find_invalid_ids_thorough(&self) -> Vec<u64> {
        self.ranges
            .iter()
            .flat_map(Range::find_invalid_thorough)
            .collect()
    }

    pub(super) fn sum_invalid_ids_thorough(&self) -> u64 {
        self.find_invalid_ids_thorough().iter().sum()
    }

    fn sum_invalid_ids(&self) -> u64 {
        self.find_invalid_ids().iter().sum()
    }
}

impl<T: AsRef<[u8]>> From<T> for Ranges {
    fn from(value: T) -> Self {
        Self {
            ranges: value
                .as_ref()
                .split(|c| *c == b',')
                .map(Range::from)
                .collect(),
        }
    }
}

#[must_use]
pub fn part1(input: &[u8]) -> u64 {
    Ranges::from(input).sum_invalid_ids()
}

#[must_use]
pub fn part2(input: &[u8]) -> u64 {
    Ranges::from(input).sum_invalid_ids_thorough()
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_range() {
        let range = Range::from(b"1-3");
        assert_eq!(range.start, 1);
        let range = Range::from(b"1-13");
        assert_eq!(range.start, 1);
        assert_eq!(range.end, 13);
        assert_eq!(range.find_invalid(), vec![11u64]);
    }
}
