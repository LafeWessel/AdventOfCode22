use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use super::Problem;

#[derive(Copy, Clone, Debug)]
pub struct Problem3_1;
impl Problem for Problem3_1 {
    fn solve(&self, file_dir: &str) {
        let file = File::open(format!("{file_dir}/3_1.txt")).unwrap();
        let reader = BufReader::new(file);

        let mut total: i32 = 0;
        for line in reader.lines() {
            let ln = line.unwrap();

            let val = self.find_char(&ln);
            let res = self.get_char_value(val);
            total += res;
        }
        println!("3-1 : Result is {}", total);
    }
}
impl Problem3_1 {
    fn get_char_value(&self, val: u8) -> i32 {
        if val > 0x60 {
            // value is lowercase
            (val - 0x60) as i32
        } else {
            // value is uppercase
            ((val - 0x40) + 26) as i32
        }
    }
    fn find_char(&self, letters: &str) -> u8 {
        // split the string in half and find the only letter that is in both sets
        let half = letters.len() / 2;
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

#[derive(Copy, Clone, Debug)]
pub struct Problem3_2;
impl Problem for Problem3_2 {
    fn solve(&self, file_dir: &str) {
        let file = File::open(format!("{file_dir}/3_1.txt")).unwrap();
        let reader = BufReader::new(&file);
        let mut total: i32 = 0;
        let _ct = reader.lines().count() / 3;

        let file = File::open(format!("{file_dir}/3_1.txt")).unwrap();
        let reader = BufReader::new(&file);
        let mut lines = reader.lines();
        loop {
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
        println!("3-2 : Result is {}", total);
    }
}
impl Problem3_2 {
    fn get_char_value(&self, val: u8) -> i32 {
        if val > 0x60 {
            // value is lowercase
            (val - 0x60) as i32
        } else {
            // value is uppercase
            ((val - 0x40) + 26) as i32
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
