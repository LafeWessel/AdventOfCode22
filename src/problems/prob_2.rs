use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use super::Problem;

enum Outcome {
    Win,
    Lose,
    Draw,
}
enum Rps {
    Rock,
    Paper,
    Scissors,
}
impl Rps {
    fn value(&self) -> i32 {
        match &self {
            Rps::Rock => 1,
            Rps::Paper => 2,
            Rps::Scissors => 3,
        }
    }
    fn rps_match(&self, b: &Self) -> i32 {
        match &self {
            Rps::Rock => match b {
                Rps::Rock => 3,
                Rps::Paper => 0,
                Rps::Scissors => 6,
            },
            Rps::Paper => match b {
                Rps::Rock => 6,
                Rps::Paper => 3,
                Rps::Scissors => 0,
            },
            Rps::Scissors => match b {
                Rps::Rock => 0,
                Rps::Paper => 6,
                Rps::Scissors => 3,
            },
        }
    }
    fn find_type_for_outcome(&self, out: &Outcome) -> Self {
        match &self {
            Rps::Rock => match out {
                Outcome::Win => Rps::Paper,
                Outcome::Lose => Rps::Scissors,
                Outcome::Draw => Rps::Rock,
            },
            Rps::Paper => match out {
                Outcome::Win => Rps::Scissors,
                Outcome::Lose => Rps::Rock,
                Outcome::Draw => Rps::Paper,
            },
            Rps::Scissors => match out {
                Outcome::Win => Rps::Rock,
                Outcome::Lose => Rps::Paper,
                Outcome::Draw => Rps::Scissors,
            },
        }
    }
}
struct Problem2_1;
impl Problem for Problem2_1 {
    fn solve(&self, file_dir: &str) {
        // read in file
        let file = File::open(format!("{file_dir}/2_1.txt")).unwrap();
        let reader = BufReader::new(file);

        let mut total: i32 = 0;
        for line in reader.lines() {
            // split line
            let ln = line.unwrap();
            let mut split = ln.split(" ");
            // determine elf's play
            let ep = self.elfs(split.next().unwrap());
            // determine my play
            let mp = self.mine(split.next().unwrap());
            // play match and add to total
            total += mp.rps_match(&ep);
            total += mp.value();
        }
        println!("2-1 : Result is {}", total);
    }
}
impl Problem2_1 {
    fn mine(&self, play: &str) -> Rps {
        match play {
            "X" => Rps::Rock,
            "Y" => Rps::Paper,
            "Z" => Rps::Scissors,
            _ => panic!("Unknown type"),
        }
    }
    fn elfs(&self, play: &str) -> Rps {
        match play {
            "A" => Rps::Rock,
            "B" => Rps::Paper,
            "C" => Rps::Scissors,
            _ => panic!("Unknown value"),
        }
    }
}

struct Problem2_2;
impl Problem for Problem2_2 {
    fn solve(&self, file_dir: &str) {
        // read in file
        let file = File::open(format!("{file_dir}/2_1.txt")).unwrap();
        let reader = BufReader::new(file);

        let mut total: i32 = 0;
        for line in reader.lines() {
            // split line
            let ln = line.unwrap();
            let mut split = ln.split(" ");
            // determine elf's play
            let ep = self.elfs(split.next().unwrap());
            // determine my play
            let mo = self.mine(split.next().unwrap());
            let mp = Rps::find_type_for_outcome(&ep, &mo);
            // play match and add to total
            total += mp.rps_match(&ep);
            total += mp.value();
        }
        println!("2-2 : Result is {}", total);
    }
}
impl Problem2_2 {
    fn mine(&self, play: &str) -> Outcome {
        match play {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Unknown type"),
        }
    }
    fn elfs(&self, play: &str) -> Rps {
        match play {
            "A" => Rps::Rock,
            "B" => Rps::Paper,
            "C" => Rps::Scissors,
            _ => panic!("Unknown value"),
        }
    }
}
