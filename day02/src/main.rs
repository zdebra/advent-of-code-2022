use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).expect("open failed");

    let mut total_score = 0;
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let (a_move, exp_res) = line.split_at(2);
        let exp_res = match exp_res.trim() {
            "X" => GameResult::PlayerAVictory,
            "Y" => GameResult::Draw,
            "Z" => GameResult::PlayerBVictory,
            _ => panic!("unexpected letter"),
        };

        let a_move = HandShape::new(a_move.trim());
        let b_move = next_move(a_move, exp_res);

        let result = play(a_move, b_move);
        let res_score = match result {
            GameResult::PlayerAVictory => 0,
            GameResult::PlayerBVictory => 6,
            GameResult::Draw => 3,
        };
        let move_score = match b_move {
            HandShape::Rock => 1,
            HandShape::Paper => 2,
            HandShape::Scissors => 3,
        };
        total_score += res_score + move_score;
    }
    println!("{}", total_score);
}

fn next_move(a_move: HandShape, exp_res: GameResult) -> HandShape {
    if exp_res == GameResult::Draw {
        return a_move;
    }
    if exp_res == GameResult::PlayerAVictory {
        return a_move.beats();
    }
    return a_move.lose_with();
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum HandShape {
    Rock,
    Paper,
    Scissors,
}

impl HandShape {
    fn new(letter: &str) -> Self {
        match letter {
            "A" | "X" => HandShape::Rock,
            "B" | "Y" => HandShape::Paper,
            "C" | "Z" => HandShape::Scissors,
            _ => panic!("unexpected letter: {}", letter),
        }
    }

    fn beats(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    fn lose_with(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }
}

#[derive(PartialEq)]
enum GameResult {
    PlayerAVictory,
    PlayerBVictory,
    Draw,
}

fn play(a: HandShape, b: HandShape) -> GameResult {
    if a == b {
        return GameResult::Draw;
    }
    if a == HandShape::Scissors && b == HandShape::Paper {
        return GameResult::PlayerAVictory;
    }
    if a == HandShape::Paper && b == HandShape::Rock {
        return GameResult::PlayerAVictory;
    }
    if a == HandShape::Rock && b == HandShape::Scissors {
        return GameResult::PlayerAVictory;
    }
    GameResult::PlayerBVictory
}
