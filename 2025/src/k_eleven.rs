pub fn solve_eleven(input: &[u8]) -> u64 {
    let input = unsafe { std::str::from_utf8_unchecked(input) };

    let operators: Vec<&str> = input
        .lines()
        .rev()
        .take(1)
        .map(|line| line.split(' ').collect::<Vec<&str>>())
        .flatten()
        .collect();

    let rows: Vec<Vec<u64>> = input
        .lines()
        .rev()
        .skip(1)
        .map(|line| {
            line.split(' ')
                .filter(|txt| !txt.is_empty())
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect();

    let mut count = 0u64;

    operators
        .iter()
        .filter(|txt| !txt.is_empty())
        .enumerate()
        .for_each(|(i, op)| {
            let itr = rows.iter().map(|col| col[i]);
            let res: u64 = match op {
                &"+" => itr.sum(),
                &"*" => itr.product(),
                _ => unreachable!("was promised + or *"),
            };

            count += res;
        });

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve_eleven() {
        let input = b"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        assert_eq!(solve_eleven(input), 4277556);
    }
}
