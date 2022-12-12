use std::fs::File;
use std::io::{self, prelude::*, BufReader, Cursor};
fn main() {
    let probs: Vec<Box<dyn problem>> = vec![
        Box::new(problem_12_1_1 {}),
        Box::new(problem_12_1_2 {}),
        Box::new(problem_12_2_1 {}),
        Box::new(problem_12_2_2 {}),
        Box::new(problem_12_3_1 {}),
        Box::new(problem_12_3_2 {}),
        Box::new(problem_12_4_1 {}),
        Box::new(problem_12_4_2 {}),
    ];
    for p in probs {
        p.solve();
    }
}

trait problem {
    fn solve(&self) -> () {
        println! {"unimplemented!"};
    }
}

struct problem_12_1_1 {}
impl problem for problem_12_1_1 {
    fn solve(&self) {
        // read in the file
        let file = File::open("12_1_1.txt").unwrap();
        let reader = BufReader::new(file);

        let mut res: Vec<i32> = vec![0];
        for line in reader.lines() {
            let l = line.unwrap();
            if !l.is_empty() {
                let last: usize = res.len() - 1;
                let num: i32 = l.parse().unwrap();
                res[last] += num;
            } else {
                res.push(0);
            }
        }

        // Print out max of vec
        println!("12-1-1: Result is: {}", res.iter().max().unwrap());
    }
}
struct problem_12_1_2 {}
impl problem for problem_12_1_2 {
    fn solve(&self) {
        // read in the file
        let file = File::open("12_1_1.txt").unwrap();
        let reader = BufReader::new(file);

        let mut res: Vec<i32> = vec![0];
        for line in reader.lines() {
            let l = line.unwrap();
            if !l.is_empty() {
                let last: usize = res.len() - 1;
                let num: i32 = l.parse().unwrap();
                res[last] += num;
            } else {
                res.push(0);
            }
        }
        // Print out max of vec
        res.sort();
        let top_3 = res.iter().skip(res.len() - 3).sum::<i32>();
        println!("12-2-2: Result is: {}", top_3);
    }
}
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

struct problem_12_2_1 {}
impl problem for problem_12_2_1 {
    fn solve(&self) {
        // read in file
        let file = File::open("12_2_1.txt").unwrap();
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
        println!("12-2-1 : Result is {}", total);
    }
}
impl problem_12_2_1 {
    fn mine(&self, play: &str) -> rps {
        match play {
            "X" => rps::rock,
            "Y" => rps::paper,
            "Z" => rps::scissors,
            _ => panic!("Unknown type"),
        }
    }
    fn elfs(&self, play: &str) -> rps {
        match play {
            "A" => rps::rock,
            "B" => rps::paper,
            "C" => rps::scissors,
            _ => panic!("Unknown value"),
        }
    }
}
struct problem_12_2_2 {}
impl problem for problem_12_2_2 {
    fn solve(&self) {
        // read in file
        let file = File::open("12_2_1.txt").unwrap();
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
            let mp = rps::find_type_for_outcome(&ep, &mo);
            // play match and add to total
            total += mp.rps_match(&ep);
            total += mp.value();
        }
        println!("12-2-2 : Result is {}", total);
    }
}
impl problem_12_2_2 {
    fn mine(&self, play: &str) -> outcome {
        match play {
            "X" => outcome::lose,
            "Y" => outcome::draw,
            "Z" => outcome::win,
            _ => panic!("Unknown type"),
        }
    }
    fn elfs(&self, play: &str) -> rps {
        match play {
            "A" => rps::rock,
            "B" => rps::paper,
            "C" => rps::scissors,
            _ => panic!("Unknown value"),
        }
    }
}

struct problem_12_3_1 {}
impl problem for problem_12_3_1 {
    fn solve(&self) {
        let file = File::open("12_3_1.txt").unwrap();
        let reader = BufReader::new(file);

        let mut total: i32 = 0;
        for line in reader.lines() {
            let ln = line.unwrap();

            let val = self.find_char(&ln);
            let res = self.get_char_value(val);
            total += res;
        }
        println!("12-3-1 : Result is {}", total);
    }
}
impl problem_12_3_1 {
    fn get_char_value(&self, val: u8) -> i32 {
        if val > 0x60 {
            // value is lowercase
            return (val - 0x60) as i32;
        } else {
            // value is uppercase
            return ((val - 0x40) + 26) as i32;
        }
    }
    fn find_char(&self, letters: &str) -> u8 {
        // split the string in half and find the only letter that is in both sets
        let half = (letters.len() / 2) as usize;
        let fw = &letters[0..half];
        let sw = &letters[half..];

        for l in fw.chars() {
            if sw.contains(l) {
                return l as u8;
            }
        }
        panic!("unable to find letter!");
    }
}
struct problem_12_3_2 {}
impl problem for problem_12_3_2 {
    fn solve(&self) {
        let file = File::open("12_3_1.txt").unwrap();
        let reader = BufReader::new(&file);
        let mut total: i32 = 0;
        let ct = reader.lines().count() / 3;
        let file = File::open("12_3_1.txt").unwrap();
        let reader = BufReader::new(&file);
        let mut lines = reader.lines();
        while true {
            let fst = match lines.next() {
                Some(s) => s.unwrap(),
                None => {
                    break;
                }
            };
            let scd = lines.next().unwrap().unwrap();
            let thd = lines.next().unwrap().unwrap();
            let letter = self.find_common_char(&fst, &scd, &thd);
            total += self.get_char_value(letter);
        }
        println!("12-3-2 : Result is {}", total);
    }
}
impl problem_12_3_2 {
    fn get_char_value(&self, val: u8) -> i32 {
        if val > 0x60 {
            // value is lowercase
            return (val - 0x60) as i32;
        } else {
            // value is uppercase
            return ((val - 0x40) + 26) as i32;
        }
    }
    fn find_common_char(&self, str1: &str, str2: &str, str3: &str) -> u8 {
        for c1 in str1.chars() {
            if str2.contains(c1) && str3.contains(c1) {
                return c1 as u8;
            }
        }
        panic!("Couldn't find string");
    }
}

struct problem_12_4_1 {}
impl problem for problem_12_4_1 {
    fn solve(&self) -> () {
        let file = File::open("12_4_1.txt").unwrap();
        let reader = BufReader::new(&file);
        let mut total: i32 = 0;
        for l in reader.lines() {
            let ln = l.unwrap();
            let spl = ln.split(",").collect::<Vec<&str>>();
            let r1 = self.find_range(spl[0]);

            let r2 = self.find_range(spl[1]);
            if self.is_inclusive(r1, r2) {
                total += 1;
            }
        }
        println! {"12-4-1 : Result is {}", total};
    }
}
impl problem_12_4_1 {
    fn find_range(&self, range: &str) -> (i32, i32) {
        let v = range.split("-").collect::<Vec<&str>>();
        let low = v[0].parse::<i32>().unwrap();
        let high = v[1].parse::<i32>().unwrap();
        (low, high)
    }
    fn is_inclusive(&self, r1: (i32, i32), r2: (i32, i32)) -> bool {
        if r1.0 == r2.0 || r1.1 == r2.1 {
            return true;
        }
        match r1.0 >= r2.0 {
            true => r1.1 <= r2.1,
            false => r1.1 >= r2.1,
        }
    }
}
struct problem_12_4_2 {}
impl problem for problem_12_4_2 {
    fn solve(&self) -> () {
        let file = File::open("12_4_1.txt").unwrap();
        let reader = BufReader::new(&file);
        let mut total: i32 = 0;
        for l in reader.lines() {
            let ln = l.unwrap();
            let spl = ln.split(",").collect::<Vec<&str>>();
            let r1 = self.find_range(spl[0]);

            let r2 = self.find_range(spl[1]);
            if self.is_overlapping(r1, r2) {
                total += 1;
            }
        }
        println! {"12-4-2 : Result is {}", total};
    }
}
impl problem_12_4_2 {
    fn find_range(&self, range: &str) -> (i32, i32) {
        let v = range.split("-").collect::<Vec<&str>>();
        let low = v[0].parse::<i32>().unwrap();
        let high = v[1].parse::<i32>().unwrap();
        (low, high)
    }
    fn is_overlapping(&self, r1: (i32, i32), r2: (i32, i32)) -> bool {
        if r1.0 == r2.0 || r1.1 == r2.1 {
            return true;
        }
        match r1.1 <= r2.1 {
            true => r1.1 >= r2.0,
            false => r1.0 <= r2.1,
        }
    }
}
