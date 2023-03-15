use std::fs::File;
use std::io::Read;

fn main() {
    // Take a file as input
    let mut buffer = String::new();
    let mut file = File::open("src/day2/resources/input.txt").expect("Unable to open file");
    file.read_to_string(&mut buffer).expect("Unable to read file");

    let mut score_game1: u32 = 0;
    let mut score_strat: u32 = 0;

    for line in buffer.lines() {
        score_game1 += get_score(line.to_uppercase().split_whitespace().collect());
        score_strat += get_strat_score(line.to_uppercase().split_whitespace().collect());
    }

    println!("Score of game: {}\nScore of game using strategy: {}", score_game1, score_strat);
}

fn get_score(split_line: Vec<&str>) -> u32 {
    let p1 = RPS::from_char(split_line[0]).unwrap();
    let p2 = RPS::from_char(split_line[1]).unwrap();
    calculate_score(p1,p2)
}

fn get_strat_score(split_line: Vec<&str>) -> u32 {
    let p1 = RPS::from_char(split_line[0]).unwrap();
    let p2 = RPS::from_move(split_line[1],p1).unwrap();
    calculate_score(p1,p2)
}

fn calculate_score(p1: RPS, p2: RPS) -> u32 {
    if p1.as_u8() ^ p2.as_u8() == 0 {
        return 3 + p2.score();
    } else {
        if p1.as_u8() ^ p2.as_u8() == 0b110 {
            return if p2.as_u8() == 0b010 { 6 + p2.score() } else { 0 + p2.score() };
        } else if p1.as_u8() ^ p2.as_u8() == 0b101 {
            return if p2.as_u8() == 0b100 { 6 + p2.score() } else { 0 + p2.score() };
        } else if p1.as_u8() ^ p2.as_u8() == 0b011 {
            return if p2.as_u8() == 0b001 { 6 + p2.score() } else { 0 + p2.score() };
        }
    }
    return 0;
}

#[derive(Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn from_char(c: &str) -> Option<RPS> {
        match c {
            "A" | "X" => Some(RPS::Rock),
            "B" | "Y" => Some(RPS::Paper),
            "C" | "Z" => Some(RPS::Scissors),
            _ => None
        }
    }

    fn from_move(c: &str, other: RPS) -> Option<RPS> {
        match c {
            "X" => {
                match other {
                    RPS::Rock => Some(RPS::Scissors),
                    RPS::Paper => Some(RPS::Rock),
                    RPS::Scissors => Some(RPS::Paper)
                }
            },
            "Y" => {
                match other {
                    RPS::Rock => Some(RPS::Rock),
                    RPS::Paper => Some(RPS::Paper),
                    RPS::Scissors => Some(RPS::Scissors)
                }
            },
            "Z" => {
                match other {
                    RPS::Rock => Some(RPS::Paper),
                    RPS::Paper => Some(RPS::Scissors),
                    RPS::Scissors => Some(RPS::Rock)
                }
            },
            _ => None
        }
    }

    fn as_u8(&self) -> u8 {
        match self {
            RPS::Rock => 0b100,
            RPS::Paper => 0b010,
            RPS::Scissors => 0b001
        }
    }

    fn score(&self) -> u32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3
        }
    }
}



