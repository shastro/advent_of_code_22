pub fn process_part1(input: &str) -> String {
    for (i, c) in input.as_bytes().windows(4).enumerate() {
        let mut counts = 0;
        for x in c {
            for y in c {
                if *x == *y {
                    counts += 1;
                }
            }
        }
        if counts == 4 {
            return (i + 4).to_string();
        }
    }
    "0".to_string()
}

pub fn process_part2(input: &str) -> String {
    for (i, c) in input.as_bytes().windows(14).enumerate() {
        let mut counts = 0;
        for x in c {
            for y in c {
                if *x == *y {
                    counts += 1;
                }
            }
        }
        if counts == 14 {
            return (i + 14).to_string();
        }
    }
    "0".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const input: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

        assert_eq!(process_part1(input), "7");
    }

    #[test]
    fn test_part2() {
        const input: &str = "nppdvjthqldpwncqszvftbrmjlhg";

        assert_eq!(process_part2(input), "23");
    }
}
