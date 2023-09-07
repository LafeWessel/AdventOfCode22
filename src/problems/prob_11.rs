use std::{io::{BufReader, BufRead}, fs::File, collections::HashMap, fmt::Debug};
use super::Problem;


pub struct Monkey {
    id: u128,
    items: Vec<u128>,
    op: Box<dyn Fn(u128) -> u128>,
    test: Box<dyn Fn(u128) -> u128>,
    inspected: u128,
    divisor: u128,

}



impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Monkey {{ id: {}, items: {:?}, inspected: {} }}", self.id, self.items, self.inspected)
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
                    Box::new(|v| v + v) as Box<dyn Fn(u128) -> u128>
                } else {
                    let k: u128 = ops[2].parse().unwrap();
                    println!("old + {k}");
                    Box::new(move |v| v + k) as Box<dyn Fn(u128) -> u128>
                }
            },
            "*" => {
                if ops[2] == "old"{
                    println!("old * old");
                    Box::new(|v| v * v) as Box<dyn Fn(u128) -> u128>
                } else {
                    let k: u128 = ops[2].parse().unwrap();
                    println!("old * {k}");
                    Box::new(move |v| v * k) as Box<dyn Fn(u128) -> u128>
                }

            }
            _ => panic!("Unknown operation")
        };

        let div = lns[3].split(' ').last().unwrap().parse::<u128>().unwrap();
        let m1 = lns[4].split(' ').last().unwrap().parse::<u128>().unwrap();
        let m2 = lns[5].split(' ').last().unwrap().parse::<u128>().unwrap();

        println!("v % {div} == 0 -> {m1}, else {m2}");
        let test = Box::new(move |v| if v % div == 0 { m1 } else { m2 } );
        Monkey {
            id,
            items,
            op,
            test,
            inspected: 0,
            divisor: div
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
                lns.push(r.clone());
            }
            if r.contains("false"){
                let mk = Self::parse_monkey(&lns);
                lns.clear();
                mkys.insert(mk.id, mk);
            }
        }

        println!("{mkys:?}");


        let mut keys = mkys.keys().cloned().collect::<Vec<_>>();
        keys.sort();

        println!("{keys:?}");

        for r in 1..=20 {
            println!("Round {r}");

            for k in keys.iter(){
                for i in mkys.get(k).unwrap().items.clone().iter() {
                    let n = (mkys.get(k).unwrap().op)(*i) / 3;
                    let to = (mkys.get(k).unwrap().test)(n);
                    println!("item: {i}, new: {n}, to: {to}");
                    mkys.get_mut(k).unwrap().inspected += 1;
                    mkys.get_mut(&to).unwrap().items.push(n);
                }
                mkys.get_mut(k).unwrap().items.clear();
            }
        }

        let mut ins = mkys.values().map(|m| m.inspected).collect::<Vec<_>>();
        ins.sort();
        
        println!("11-1: {}", ins[ins.len() - 1] * ins[ins.len() - 2]);
    }
}

pub struct Problem11_2;

impl Problem for Problem11_2 {
    fn solve(&self, file_dir: &str) {
        let file = File::open(&format!("{file_dir}/11_1.txt")).unwrap();
        let rdr = BufReader::new(file);
        let mut mkys = HashMap::new();

        let mut lns = vec![];
        for r in rdr.lines() {
            let r = r.unwrap();
            if !r.is_empty(){
                lns.push(r.clone());
            }
            if r.contains("false"){
                let mk = Problem11_1::parse_monkey(&lns);
                lns.clear();
                mkys.insert(mk.id, mk);
            }
        }

        println!("{mkys:?}");


        let mut keys = mkys.keys().cloned().collect::<Vec<_>>();
        keys.sort();

        println!("{keys:?}");

        let div = mkys.values().map(|d| d.divisor).product::<u128>();
        println!("Divisor: {div}");

        for r in 1..=10_000 {
            println!("Round {r}");

            for k in keys.iter(){
                for i in mkys.get(k).unwrap().items.clone().iter() {
                    let n = (mkys.get(k).unwrap().op)(*i) % div;
                    let to = (mkys.get(k).unwrap().test)(n);
                    mkys.get_mut(k).unwrap().inspected += 1;
                    mkys.get_mut(&to).unwrap().items.push(n);
                }
                mkys.get_mut(k).unwrap().items.clear();
            }
        }

        let mut ins = mkys.values().map(|m| m.inspected).collect::<Vec<_>>();
        ins.sort();
        
        println!("11-1: {}", ins[ins.len() - 1] * ins[ins.len() - 2]);
    }
}
