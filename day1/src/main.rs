use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Cannot read file");

    let mut summed_calories: Vec<i32> = contents
        .split("\n\n")
        .map(|arr| {
            arr.lines()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<i32>>()
                .into_iter()
                .sum()
        })
        .collect::<Vec<i32>>();

    summed_calories.sort_by(|a, b| b.cmp(a));

    println!("Items {:?}", &summed_calories[..3].iter().sum::<i32>());
}
