use std::{io::{BufReader, BufRead}, fs::File, collections::HashMap, fmt::Debug};

use super::Problem;


pub struct Monkey {
    id: u32,
    items: Vec<u32>,
    op: Box<dyn Fn(u32) -> u32>,
    test: Box<dyn Fn(u32) -> usize>,
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Monkey: {}, {:?}", self.id, self.items)
    }
}


pub struct Problem11_1;

impl Problem11_1 {

    fn parse_monkey(lns: &[String]) -> Monkey {
        let id = lns[0].split(' ').nth(1).unwrap().replace(':', "").parse().unwrap();
        let items = lns[1].split(':').nth(1).unwrap().split(',').map(|v| v.trim().parse().unwrap()).collect();
        let ops = lns[2].split('=').nth(1).unwrap().split(' ').skip(1).map(|v| v.trim().to_owned()).collect::<Vec<String>>();
        let op = match &*ops[1] {
            "+" => {
                if ops[2] == "old"{
                    println!("old + old");
                    Box::new(|v| v + v) as Box<dyn Fn(u32) -> u32>
                } else {
                    let k: u32 = ops[2].parse().unwrap();
                    println!("old + {k}");
                    Box::new(move |v| v + k) as Box<dyn Fn(u32) -> u32>
                }
            },
            "*" => {
                if ops[2] == "old"{
                    println!("old * old");
                    Box::new(|v| v * v) as Box<dyn Fn(u32) -> u32>
                } else {
                    let k: u32 = ops[2].parse().unwrap();
                    println!("old * {k}");
                    Box::new(move |v| v * k) as Box<dyn Fn(u32) -> u32>
                }

            }
            _ => panic!("Unknown operation")
        };

        let div = lns[3].split(' ').last().unwrap().parse::<u32>().unwrap();
        let m1 = lns[4].split(' ').last().unwrap().parse::<usize>().unwrap();
        let m2 = lns[5].split(' ').last().unwrap().parse::<usize>().unwrap();

        println!("v % {div} == 0 -> {m1}, else {m2}");
        let test = Box::new(move |v| if v % div == 0 { m1 } else { m2 } );
        Monkey {
            id,
            items,
            op,
            test
        }
    }
}

impl Problem for Problem11_1{
    fn solve(&self, file_dir: &str) {
        let file = File::open(&format!("{file_dir}/11_1.txt")).unwrap();
        let rdr = BufReader::new(file);
        let mut mkys = HashMap::new();

        let mut lns = vec![];
        for r in rdr.lines() {
            let r = r.unwrap();
            if !r.is_empty(){
                lns.push(r);
            }
            else {
                let mk = Self::parse_monkey(&lns);
                lns.clear();
                mkys.insert(mk.id, mk);
            }
        }

        println!("{mkys:?}");



    }
}

pub struct Problem11_2;
impl Problem for Problem11_2 {
    fn solve(&self, file_dir: &str) {
        todo!()
    }
}
