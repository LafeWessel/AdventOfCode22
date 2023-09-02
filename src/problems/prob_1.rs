use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use super::Problem;

struct Problem1_1;
impl Problem for Problem1_1 {
    fn solve(&self, file_dir: &str) {
        // read in the file
        let file = File::open(format!("{file_dir}/1_1.txt")).unwrap();
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
        println!("1-1: Result is: {}", res.iter().max().unwrap());
    }
}

struct Problem1_2;
impl Problem for Problem1_2 {
    fn solve(&self, file_dir: &str) {
        // read in the file
        let file = File::open(format!("{file_dir}/1_1.txt")).unwrap();
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
        println!("1-2: Result is: {}", top_3);
    }
}
