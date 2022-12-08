use std::fs;
use day3::process_part1;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Cannot read file");
    println!("{}", process_part1(&contents))
}
