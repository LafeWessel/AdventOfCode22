use std::{io::{BufReader, BufRead}, fs::File, collections::HashMap};

use itertools::Itertools;

use super::Problem;

pub struct Problem10_1;

#[derive(Copy, Clone, Debug)]
pub enum Operation {
    NoOp,
    AddX(i64)
}

impl Problem10_1 {
    pub fn parse_line(ln: &str) -> Operation {
        let mut spl = ln.split(' ');
        if spl.next().unwrap() == "addx"{
            Operation::AddX(spl.next().unwrap().parse().unwrap())
        } else {
            Operation::NoOp
        }
    }

    fn check_signal(map: &mut HashMap<usize, i64>, clk: usize, acc: i64){
        if map.keys().contains(&clk) {
            println!("{clk}: {acc} => {}", acc * clk as i64);
            map.entry(clk).and_modify(|e| {*e += acc * clk as i64});
        }
    }
}

impl Problem for Problem10_1 {
    fn solve(&self, file_dir: &str) {
        let file  = File::open(&format!("{file_dir}/10_1.txt")).unwrap();
        let rdr = BufReader::new(file);

        let mut map = HashMap::from([
            (20, 0),
            (60, 0),
            (100, 0),
            (140, 0),
            (180, 0),
            (220, 0)]);

        let mut clk = 0;
        let mut acc = 1;
        for l in rdr.lines() {
            let ln = l.unwrap();
            let op = Self::parse_line(&ln);
            println!("{op:?}");

            match op {
                Operation::NoOp =>  {
                    clk += 1;
                    Self::check_signal(&mut map, clk, acc);
                }
                Operation::AddX(v) =>  {
                    clk += 1;
                    Self::check_signal(&mut map, clk, acc);
                    clk += 1;
                    Self::check_signal(&mut map, clk, acc);
                    acc += v;
                }
            }
        }

        let sm = map.values().sum::<i64>();
        println!("10-1: {sm}");
    }
}

pub struct Problem10_2;

impl Problem10_2 {

}

impl Problem for Problem10_2 {
    fn solve(&self, file_dir: &str) {
        let file  = File::open(&format!("{file_dir}/10_1.txt")).unwrap();
        let rdr = BufReader::new(file);

        let mut instructions = rdr.lines().map(|ln| Problem10_1::parse_line(&ln.unwrap()));
        let mut screen = vec![vec![false; 40]; 6];
        let mut sprite_pos = 1;

        let mut cur = None;

        for (k, r) in screen.iter_mut().enumerate() {
            for (i, j) in r.iter_mut().enumerate() {
                print!("{k}: {i}: ");
                // draw
                if (i as i32 - sprite_pos).abs() <= 1{
                    println!("Drawing!");
                    *j = true;
                } else{
                    println!("Not drawing")
                }

                match cur {
                    Some(op) => {
                        // finish addx
                        match op {
                            Operation::NoOp =>  {
                                panic!("Somehow had NoOp at start of iteration");
                            }
                            Operation::AddX(v) => {
                                println!("Adding {v} to {sprite_pos}");
                                sprite_pos += v as i32;
                                cur = None;
                            }
                        }
                    },
                    None => {
                        // start instruction
                        cur = instructions.next();

                        if let Some(Operation::NoOp) = cur {
                            cur = None;
                            println!("No op")
                        }
                    }
                }
            }
        }

        for r in screen.iter() {
            for j in r.iter() {
                match j {
                    true => print!("# "),
                    false => print!(". "),
                }
            }
            println!();
        }
    }
}
