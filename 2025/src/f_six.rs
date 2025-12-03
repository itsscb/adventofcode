use crate::e_five::BatteryPack;

use super::e_five::Bank;

impl Bank {
    fn overdrive_joltage(&self) -> u64 {
        // let mut max = 0u64;
        // let mut result = String::new();
        // self.0.chars().enumerate().for_each(|(i, a)| {
        //     if let Some(b) = self.0.chars().skip(i + 1).max() {
        //         let tmp = format!("{}{}", a.to_digit(10).unwrap(), b.to_digit(10).unwrap());
        //         let sum = tmp.parse::<u64>().unwrap();
        //         if sum > max {
        //             result = tmp;
        //             max = sum;
        //         }
        //     }
        // });
        // max
        overdrive_joltage(&self.0, 12).parse::<u64>().unwrap()
    }
}

fn overdrive_joltage<T: AsRef<str>>(batteries: T, window: usize) -> String {
    // println!("root: {}:{window}", batteries.as_ref());
    let mut max = 0u64;
    let mut result = String::new();
    for (i, c) in batteries.as_ref().chars().enumerate() {
        let mut tmp = String::from(c);
        for j in (0..window).rev() {
            let b = batteries
                .as_ref()
                .chars()
                .skip(i + window - j + 1)
                .collect::<String>();
            overdrive_joltage(b, j).chars().for_each(|c| {
                tmp.push(c);
            });
        }
        if tmp.is_empty() {
            continue;
        }
        let sum = tmp.parse::<u64>().unwrap();
        if sum > max {
            result = tmp;
            max = sum;
        }
    }

    result
}

impl BatteryPack {
    fn sum_of_overdrive_joltage(&self) -> u128 {
        self.banks
            .iter()
            .map(|b| u128::from(b.overdrive_joltage()))
            .sum()
    }
}

pub fn solve_six(input: &[u8]) -> u128 {
    let pack = BatteryPack::from(input);
    pack.sum_of_overdrive_joltage()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bank() {
        let bank = Bank::from(b"987654321111111");
        assert_eq!(bank.overdrive_joltage(), 987654321111);

        let bank = Bank::from(b"811111111111119");
        assert_eq!(bank.overdrive_joltage(), 811111111119);
    }
}
