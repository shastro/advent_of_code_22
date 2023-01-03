use day4::process_part1;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Cannot read file");
    println!("{}", process_part1(&contents))
}
