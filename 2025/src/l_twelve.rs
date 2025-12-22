pub fn solve_twelve(input: &[u8]) -> u64 {
    let input = unsafe { std::str::from_utf8_unchecked(input) };

    let operators: Vec<&str> = input
        .lines()
        .rev()
        .take(1)
        .map(|line| line.split("").collect::<Vec<&str>>())
        .flatten()
        .filter(|i| !i.is_empty())
        .collect();

    let rows: Vec<Vec<&str>> = input
        .lines()
        .rev()
        .skip(1)
        .map(|line| {
            line.split("")
                // .filter(|txt| !txt.is_empty())
                // .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<&str>>()
        })
        .map(|line| line[1..line.len()].into())
        .collect();

    let mut tasks: Vec<(Vec<u64>, &str)> = operators.iter().map(|op| (vec![], *op)).collect();

    let len = rows.len();
    let col_len = rows[0].len();
    for x in (0..col_len).rev() {
        let mut builder: String = String::with_capacity(20);
        for y in (0..len).rev() {
            let next_num = rows[y][x];

            if next_num.is_empty() || next_num == " " {
                continue;
            }

            builder.push_str(next_num);
        }
        if builder.is_empty() {
            continue;
        }
        tasks[x].0.push(builder.parse::<u64>().unwrap());
    }

    let len = tasks.len();
    let mut op = "";
    let mut intermed: Vec<u64> = Vec::with_capacity(1_000);
    let mut count = 0u64;
    for i in 0..len {
        let o = tasks[i].1;
        if !o.is_empty() && o != " " {
            op = o;
        }
        let j = i + 1;

        if o == " " && (j < len && tasks[j].1 != " " && !tasks[j].1.is_empty()) {
            let res: u64 = match op {
                "+" => intermed.iter().sum(),
                "*" => intermed.iter().product(),
                _ => unreachable!(),
            };
            count += res;
            intermed.clear();
            continue;
        }
        intermed.extend(tasks[i].0.clone());
    }
    let res: u64 = match op {
        "+" => intermed.iter().sum(),
        "*" => intermed.iter().product(),
        _ => unreachable!(),
    };
    count += res;

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve_twelve() {
        let input = b"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        assert_eq!(solve_twelve(input), 3263827);
    }
}
