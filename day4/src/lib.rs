#[derive(Debug, Copy, Clone)]
struct Section {
    start: i32,
    end: i32,
}

fn overlap(a: Section, b: Section) -> i32 {
    let top = b.end.min(a.end);
    let bot = b.start.max(a.start);
    let length = top - bot;
    if length >= 0 {
        length + 1
    } else {
        0
    }
}

fn is_contained(overlap: i32, a: Section, b: Section) -> i32 {
    let length_right = a.end - a.start + 1;
    let length_left = b.end - b.start + 1;
    let min_len = length_left.min(length_right);
    println!("Min len{}", min_len);
    println!("Olap {}", overlap);
    (overlap == min_len) as i32
}
pub fn process_part1(input: &str) -> String {
    // Compute how many pairs exist such that one
    // range fully contains the other
    let answer: String = input
        .lines()
        .map(|l| {
            l.split(|c| vec![',', '-'].contains(&c))
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|a| {
            (
                Section {
                    start: a[0],
                    end: a[1],
                },
                Section {
                    start: a[2],
                    end: a[3],
                },
            )
        })
        .inspect(|a| println!("{:?}", a))
        .map(|(a, b)| is_contained(overlap(a, b), a, b))
        .inspect(|a| println!("{:?}", a))
        .sum::<i32>()
        .to_string();

    answer
}

pub fn process_part2(input: &str) -> String {
    let answer: String = input
        .lines()
        .map(|l| {
            l.split(|c| vec![',', '-'].contains(&c))
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|a| {
            (
                Section {
                    start: a[0],
                    end: a[1],
                },
                Section {
                    start: a[2],
                    end: a[3],
                },
            )
        })
        .inspect(|a| println!("{:?}", a))
        .map(|(a, b)| (overlap(a, b) > 0) as i32)
        .inspect(|a| println!("{:?}", a))
        .sum::<i32>()
        .to_string();

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../test_input");

    #[test]
    fn test_part1() {
        assert_eq!(process_part1(INPUT), "2");
    }

    #[test]
    fn test_part2() {
        assert_eq!(process_part2(INPUT), "4");
    }
}
