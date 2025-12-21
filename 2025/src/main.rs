#![allow(clippy::unwrap_used)]
use std::{fs::File, io::Read};

use aoc25::{
    a_one::solve_one, b_two::solve_two, c_three::solve_three, d_four::solve_four,
    e_five::solve_five, f_six::solve_six, g_seven::solve_seven, h_eight::solve_eight,
};

fn main() {
    if let Ok(mut input) = File::open("inputs/one.txt") {
        let mut buf: Vec<u8> = Vec::with_capacity(10_000);
        input.read_to_end(&mut buf).unwrap();
        let result = solve_one(&buf);
        println!("Result One: {result}");

        let result = solve_two(&buf);
        println!("Result Two: {result}");
    }

    if let Ok(mut input) = File::open("inputs/three.txt") {
        let mut buf: Vec<u8> = Vec::with_capacity(10_000);
        input.read_to_end(&mut buf).unwrap();
        let result = solve_three(&buf);
        println!("Result Three: {result}");
        let result = solve_four(&buf);
        println!("Result Four: {result}");
    }

    if let Ok(mut input) = File::open("inputs/five.txt") {
        let mut buf: Vec<u8> = Vec::with_capacity(10_000);
        input.read_to_end(&mut buf).unwrap();
        let result = solve_five(&buf);
        println!("Result Five: {result}");
        let result = solve_six(&buf);
        println!("Result Six: {result}");
    }

    if let Ok(mut input) = File::open("inputs/seven.txt") {
        let mut buf: Vec<u8> = Vec::with_capacity(10_000);
        input.read_to_end(&mut buf).unwrap();
        let result = solve_seven(&buf);
        println!("Result Seven: {result}");
        let result = solve_eight(&buf);
        println!("Result Eight: {result}");
    }
}
