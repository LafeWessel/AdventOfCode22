use std::fs::File;
use std::io::{self, prelude::*, BufReader, Cursor};

mod problems;

fn main() {}

enum outcome {
    win,
    lose,
    draw,
}
enum rps {
    rock,
    paper,
    scissors,
}
impl rps {
    fn value(&self) -> i32 {
        match &self {
            rps::rock => 1,
            rps::paper => 2,
            rps::scissors => 3,
        }
    }
    fn rps_match(&self, b: &Self) -> i32 {
        match &self {
            rps::rock => match b {
                rps::rock => 3,
                rps::paper => 0,
                rps::scissors => 6,
            },
            rps::paper => match b {
                rps::rock => 6,
                rps::paper => 3,
                rps::scissors => 0,
            },
            rps::scissors => match b {
                rps::rock => 0,
                rps::paper => 6,
                rps::scissors => 3,
            },
        }
    }
    fn find_type_for_outcome(&self, out: &outcome) -> Self {
        match &self {
            rps::rock => match out {
                outcome::win => rps::paper,
                outcome::lose => rps::scissors,
                outcome::draw => rps::rock,
            },
            rps::paper => match out {
                outcome::win => rps::scissors,
                outcome::lose => rps::rock,
                outcome::draw => rps::paper,
            },
            rps::scissors => match out {
                outcome::win => rps::rock,
                outcome::lose => rps::paper,
                outcome::draw => rps::scissors,
            },
        }
    }
}
