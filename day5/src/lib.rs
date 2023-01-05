use chumsky::prelude::*;
use chumsky::primitive::*;
use std::fs;

pub fn process_part1(input: &str) -> String {
    "works".to_string()
}

pub fn process_part2(input: &str) -> String {
    "works".to_string()
}

type Stack = Vec<char>;

#[derive(Debug, PartialEq, Eq)]
struct Instruction {
    count: u32,
    source: u32,
    destination: u32,
}

#[derive(Debug, PartialEq)]
struct CraneProblem {
    instructions: Vec<Instruction>,
    memory: Vec<Stack>,
}

fn instruction_parser() -> impl Parser<char, Instruction, Error = Simple<char>> {
    let int = text::int(10).map(|s: String| s.parse().unwrap()).padded();
    let instruction = just("move")
        .ignore_then(int)
        .then_ignore(just("from"))
        .then(int)
        .then_ignore(just("to"))
        .then(int)
        .map(|((num, src), dest)| Instruction {
            count: num,
            source: src,
            destination: dest,
        });
    instruction
}

fn file_parser() -> impl Parser<char, CraneProblem, Error = Simple<char>> {
    let int = text::int(10)
        .map(|s: String| s.parse::<i32>().unwrap())
        .padded();
    let instruction = instruction_parser();
    let cargo_box = one_of::<_, _, Simple<char>>("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
        .padded_by(just('[').or(just(']')))
        .or(just("   ").to('x'));
    let stack_ids = int.repeated().at_least(1);
    let box_line = cargo_box
        .separated_by(just(' '))
        .then_ignore(text::newline());
    let stack_parser = box_line.repeated().at_least(1).then_ignore(stack_ids);
    let file_parser = stack_parser
        .then(instruction.repeated())
        .then_ignore(end())
        .map(|(stacks, instructions)| {
            let mut new_stacks: Vec<Stack> = vec![];
            for (i, row) in stacks.iter().enumerate() {
                for (j, &val) in row.iter().enumerate() {
                    if i == 0 {
                        new_stacks.push(vec![])
                    }
                    if val != 'x' {
                        new_stacks[j].push(val);
                    }
                }
            }
            for s in new_stacks.iter_mut() {
                s.reverse();
            }
            CraneProblem {
                memory: new_stacks,
                instructions: instructions,
            }
        });

    file_parser
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        // Testing Instruction Parser
        let input = "move 1 from 3 to 2";
        let parser = instruction_parser();
        assert_eq!(
            parser.parse(input),
            Ok(Instruction {
                count: 1,
                source: 3,
                destination: 2
            })
        );

        // Testing stack_ids
        let int = text::int::<_, Simple<char>>(10)
            .map(|s: String| s.parse().unwrap())
            .padded();
        let stack_ids = int.repeated().at_least(1);
        assert_eq!(stack_ids.parse(" 1   2   3 \n"), Ok(vec![1, 2, 3]));

        // Testing cargo box parser
        let cargo_box = one_of::<_, _, Simple<char>>("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
            .padded_by(just('[').or(just(']')))
            .or(just("   ").to('x'));
        assert_eq!(cargo_box.parse("[C]"), Ok('C'));
        assert_eq!(cargo_box.parse("   "), Ok('x'));

        // Test box line parsing
        let box_line = cargo_box
            .separated_by(just(' '))
            .then_ignore(text::newline());
        assert_eq!(box_line.parse("[Z] [M] [P]\n [K]"), Ok(vec!['Z', 'M', 'P']));

        // Test stack parsing
        let test_stack = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 
        ";
        let stack_parser = box_line.repeated().at_least(1).then_ignore(stack_ids);
        assert_eq!(
            stack_parser.parse(test_stack),
            Ok(vec![
                vec!['x', 'D', 'x'],
                vec!['N', 'C', 'x'],
                vec!['Z', 'M', 'P']
            ]),
        );

        // Test Full parser
        let input = fs::read_to_string("test.txt").expect("Cannot read file");
        let fparser = file_parser();
        assert_eq!(
            fparser.parse(input),
            Ok(CraneProblem {
                memory: vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
                instructions: vec![
                    Instruction {
                        count: 1,
                        source: 2,
                        destination: 1
                    },
                    Instruction {
                        count: 3,
                        source: 1,
                        destination: 3
                    },
                    Instruction {
                        count: 2,
                        source: 2,
                        destination: 1
                    },
                    Instruction {
                        count: 1,
                        source: 1,
                        destination: 2
                    }
                ]
            })
        );
    }

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("test.txt").expect("Cannot read file");
        assert_eq!(process_part1(&input), "works");
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("test.txt").expect("Cannot read file");
        assert_eq!(process_part2(&input), "works");
    }
}
