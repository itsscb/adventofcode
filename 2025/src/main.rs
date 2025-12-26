#![allow(clippy::unwrap_used)]
use std::{fs::File, io::Read};

use aoc25::{day1, day2, day3, day4, day5, day6, day7};

fn main() {
    if let Ok(mut input) = File::open("inputs/day1.txt") {
        let mut buf: Vec<u8> = Vec::with_capacity(10_000);
        input.read_to_end(&mut buf).unwrap();
        let result = day1::part1(&buf);
        println!("Result Day 1 Part 1: {result}");

        let result = day1::part2(&buf);
        println!("Result Day 1 Part 2: {result}");
    }

    if let Ok(mut input) = File::open("inputs/day2.txt") {
        let mut buf: Vec<u8> = Vec::with_capacity(10_000);
        input.read_to_end(&mut buf).unwrap();
        let result = day2::part1(&buf);
        println!("Result Day 2 Part 1: {result}");
        let result = day2::part2(&buf);
        println!("Result Day 2 Part 2: {result}");
    }

    if let Ok(mut input) = File::open("inputs/day3.txt") {
        let mut buf: Vec<u8> = Vec::with_capacity(10_000);
        input.read_to_end(&mut buf).unwrap();
        let result = day3::part1(&buf);
        println!("Result Day 3 Part 1: {result}");
        let result = day3::part2(&buf);
        println!("Result Day 3 Part 2: {result}");
    }

    if let Ok(mut input) = File::open("inputs/day4.txt") {
        let mut buf: Vec<u8> = Vec::with_capacity(10_000);
        input.read_to_end(&mut buf).unwrap();
        let result = day4::part1(&buf);
        println!("Result Day 4 Part 1: {result}");
        let result = day4::part2(&buf);
        println!("Result Day 4 Part 2: {result}");
    }

    if let Ok(mut input) = File::open("inputs/day5.txt") {
        let mut buf: Vec<u8> = Vec::with_capacity(10_000);
        input.read_to_end(&mut buf).unwrap();
        let result = day5::part1(&buf);
        println!("Result Day 5 Part 1: {result}");
        let result = day5::part2(&buf);
        println!("Result Day 5 Part 2: {result}");
    }

    if let Ok(mut input) = File::open("inputs/day6.txt") {
        let mut buf: Vec<u8> = Vec::with_capacity(10_000);
        input.read_to_end(&mut buf).unwrap();
        let result = day6::part1(&buf);
        println!("Result Day 6 Part 1: {result}");
        let result = day6::part2(&buf);
        println!("Result Day 6 Part 2: {result}");
    }

    if let Ok(mut input) = File::open("inputs/day7.txt") {
        let mut buf: Vec<u8> = Vec::with_capacity(10_000);
        input.read_to_end(&mut buf).unwrap();
        let result = day7::part1(&buf);
        println!("Result Day 7 Part 1: {result}");
        let result = day7::part2(&buf);
        println!("Result Day 7 Part 2: {result}");
    }
}
