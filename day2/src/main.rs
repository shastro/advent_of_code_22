use std::fs;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
enum Play {
    R,
    P,
    S,
}

#[derive(Debug, Copy, Clone)]
struct ParsePlayError;

impl FromStr for Play {
    type Err = ParsePlayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Play::R),
            "Y" => Ok(Play::P),
            "Z" => Ok(Play::S),
            "A" => Ok(Play::R),
            "B" => Ok(Play::P),
            "C" => Ok(Play::S),
            _ => Err(ParsePlayError),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum GameResult {
    Win = 6,
    Lose = 0,
    Tie = 3,
}

#[derive(Debug, Copy, Clone)]
struct ParseGameResultError;

impl FromStr for GameResult {
    type Err = ParseGameResultError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(GameResult::Lose),
            "Y" => Ok(GameResult::Tie),
            "Z" => Ok(GameResult::Win),
            _ => Err(ParseGameResultError),
        }
    }
}

enum Either {
    Play(Play),
    GameResult(GameResult),
}

#[derive(Debug)]
struct EitherParseError;
impl FromStr for Either {
    type Err = EitherParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" | "Y" | "Z" => Ok(Either::GameResult(s.parse().unwrap())),
            "A" | "B" | "C" => Ok(Either::Play(s.parse().unwrap())),
            _ => Err(EitherParseError),
        }
    }
}

fn play1(my_play: Play, opp: Play) -> GameResult {
    match my_play {
        Play::R => match opp {
            Play::R => GameResult::Tie,
            Play::P => GameResult::Lose,
            Play::S => GameResult::Win,
        },
        Play::P => match opp {
            Play::R => GameResult::Win,
            Play::P => GameResult::Tie,
            Play::S => GameResult::Lose,
        },
        Play::S => match opp {
            Play::R => GameResult::Lose,
            Play::P => GameResult::Win,
            Play::S => GameResult::Tie,
        },
    }
}

fn play2(opp: Play, result: GameResult) -> Play {
    match opp {
        Play::R => match result {
            GameResult::Win => Play::P,  // Win
            GameResult::Tie => Play::R,  // Draw
            GameResult::Lose => Play::S, // Lose
        },
        Play::P => match result {
            GameResult::Win => Play::S,
            GameResult::Tie => Play::P,
            GameResult::Lose => Play::R,
        },
        Play::S => match result {
            GameResult::Win => Play::R,
            GameResult::Tie => Play::S,
            GameResult::Lose => Play::P,
        },
    }
}


fn score(my_play: Play, result: GameResult) -> i32 {
    let mut score: i32 = match my_play {
        Play::R => 1,
        Play::P => 2,
        Play::S => 3,
    };

    score += match result {
        GameResult::Win => 6,
        GameResult::Lose => 0,
        GameResult::Tie => 3,
    };

    score
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Cannot read file");
    // Part 1
    let total_score_part1: i32 = contents
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|token| token.parse().unwrap())
                .collect()
        })
        .map(|a: Vec<Play>| score(a[1], play1(a[1], a[0])))
        .sum();

    // Part 2
    let total_score_part2: i32 = contents
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|token| token.parse().unwrap())
                .collect::<Vec<Either>>()
        })
        .map(|a: Vec<Either>| {
            if let Either::Play(opp) = a[0] {
                if let Either::GameResult(res) = a[1] {
                    score(play2(opp, res), res)
                } else {
                    0
                }
            } else {
                0
            }
        })
        .sum();

    println!("{:?}", total_score_part1);
    println!("{:?}", total_score_part2);
}
