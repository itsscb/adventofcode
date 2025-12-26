#[derive(Debug, Clone)]
pub(super) struct Bank(pub(super) String);

impl<T: AsRef<[u8]>> From<T> for Bank {
    fn from(value: T) -> Self {
        // SAFETY: We know that the input is valid UTF-8.
        let v = unsafe { std::str::from_utf8_unchecked(value.as_ref()) };

        Self(String::from(v))
    }
}

impl Bank {
    #[allow(clippy::unwrap_used)]
    fn tweak_joltage(&self) -> u8 {
        let mut max = 0u32;
        self.0.chars().enumerate().for_each(|(i, a)| {
            if let Some(b) = self.0.chars().skip(i + 1).max() {
                let tmp = format!("{}{}", a.to_digit(10).unwrap(), b.to_digit(10).unwrap());
                let sum = tmp.parse::<u32>().unwrap();
                if sum > max {
                    max = sum;
                }
            }
        });
        u8::try_from(max).unwrap_or(0)
    }
}

pub(super) struct BatteryPack {
    pub(super) banks: Vec<Bank>,
}

impl<T: AsRef<[u8]>> From<T> for BatteryPack {
    fn from(value: T) -> Self {
        let v: Vec<_> = value
            .as_ref()
            .split(|c| *c == b'\n' || *c == b'\r')
            .map(Bank::from)
            .collect();
        Self { banks: v }
    }
}

impl BatteryPack {
    fn sum_of_tweaked_joltage(&self) -> u128 {
        self.banks
            .iter()
            .map(|b| u128::from(b.tweak_joltage()))
            .sum()
    }
}

#[must_use]
pub fn part1(input: &[u8]) -> u128 {
    let pack = BatteryPack::from(input);
    pack.sum_of_tweaked_joltage()
}

#[must_use]
pub fn part2(input: &[u8]) -> u64 {
    // SAFETY: input is valid UTF-8
    let input = unsafe { std::str::from_utf8_unchecked(input) };
    input
        .lines()
        .map(|line| {
            let mut window: [u8; 13] = [0; 13];
            for bytes in line.bytes() {
                window[12] = bytes - b'0';
                for index in 0..12 {
                    if window[index] < window[index + 1] {
                        window.copy_within(index + 1..13, index);
                        break;
                    }
                }
            }
            window[0..12]
                .iter()
                .fold(0u64, |a, &b| 0b1010 * a + u64::from(b))
        })
        .sum::<u64>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bank() {
        let bank = Bank::from(b"987654321111111");
        assert_eq!(bank.tweak_joltage(), 98);

        let bank = Bank::from(b"811111111111119");
        assert_eq!(bank.tweak_joltage(), 89);
    }
}
