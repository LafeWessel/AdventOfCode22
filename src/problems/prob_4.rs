use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use super::Problem;

struct Problem4_1;
impl Problem for Problem4_1 {
    fn solve(&self, file_dir: &str) -> () {
        let file = File::open(format!("{file_dir}/4_1.txt")).unwrap();
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
        println! {"4-1 : Result is {}", total};
    }
}
impl Problem4_1 {
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

struct Problem4_2;
impl Problem for Problem4_2 {
    fn solve(&self, file_dir: &str) -> () {
        let file = File::open(format!("{file_dir}/4_1.txt")).unwrap();
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
        println! {"4-2 : Result is {}", total};
    }
}
impl Problem4_2 {
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
