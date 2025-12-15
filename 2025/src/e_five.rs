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
    fn tweak_joltage(&self) -> u8 {
        let mut max = 0u32;
        let mut result = String::new();
        self.0.chars().enumerate().for_each(|(i, a)| {
            if let Some(b) = self.0.chars().skip(i + 1).max() {
                let tmp = format!("{}{}", a.to_digit(10).unwrap(), b.to_digit(10).unwrap());
                let sum = tmp.parse::<u32>().unwrap();
                if sum > max {
                    result = tmp;
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

pub fn solve_five(input: &[u8]) -> u128 {
    let pack = BatteryPack::from(input);
    pack.sum_of_tweaked_joltage()
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
