use std::{fs::File, io::Read};

use aoc25::{one::solve_one, two::solve_two};

fn main() {
    if let Ok(mut input) = File::open("inputs/one.txt") {
        let mut buf: Vec<u8> = Vec::with_capacity(10_000);
        input.read_to_end(&mut buf).unwrap();
        let result = solve_one(&buf);
        println!("Result One: {result}");

        let result = solve_two(&buf);
        println!("Result One: {result}");
    }
}
