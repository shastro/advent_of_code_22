fn find_duplicates<'a>(first: &'a str, second: &str) -> char {
    first
        .chars()
        .filter(|c| second.contains(*c))
        .next()
        .unwrap()
}

fn priority(character: char) -> i32 {
    if character.is_ascii_lowercase() {
        (character as i32) - ('a' as i32) + 1
    } else if character.is_ascii_uppercase() {
        (character as i32) - ('A' as i32) + 27
    } else {
        panic!();
    }
}

pub fn process_part1(input: &str) -> String {
    let answer: String = input
        .lines()
        .map(|s| find_duplicates(&s[0..s.len() / 2], &s[s.len() / 2..]))
        .map(|c| priority(c))
        .sum::<i32>()
        .to_string();

    answer
}

fn find_common_item(lines: (&str, &str, &str)) -> char {
    let (a, b, c) = lines;
    a.chars()
        .filter(|k| b.contains(*k))
        .filter(|k| c.contains(*k))
        .next()
        .unwrap()
}

pub fn process_part2(input: &str) -> String {
    input
        .lines()
        .zip(input.lines().skip(1))
        .zip(input.lines().skip(2))
        .step_by(3)
        .map(|((a, b), c)| find_common_item((a, b, c)))
        .map(|c| priority(c))
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const input: &str = "\
            vJrwpWtwJgWrhcsFMMfFFhFp\n\
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
            PmmdzqPrVvPwwTWBwg\n\
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
            ttgJtRGJQctTZtZT\n\
            CrZsJsPPZsGzwwsLwLmpwMDw\
        ";

        assert_eq!(process_part1(input), "157");
    }

    #[test]
    fn test_part2() {
        const input: &str = "\
            vJrwpWtwJgWrhcsFMMfFFhFp\n\
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
            PmmdzqPrVvPwwTWBwg\n\
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
            ttgJtRGJQctTZtZT\n\
            CrZsJsPPZsGzwwsLwLmpwMDw\
        ";

        assert_eq!(process_part2(input), "70");
    }
}
