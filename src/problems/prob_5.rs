use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

use super::Problem;

/// read queues from the input
fn find_queues(file_path: &str) -> HashMap<usize, VecDeque<char>> {
    let file = File::open(file_path).unwrap();
    let rdr = BufReader::new(file);
    let mut map: HashMap<usize, VecDeque<char>> = HashMap::new();

    for ln in rdr.lines() {
        let ln = ln.unwrap();
        if ln.is_empty() {
            break;
        }
        for (i, chr) in ln.chars().enumerate() {
            if i % 4 == 0 && chr == '[' {
                let c = ln.chars().nth(i + 1).unwrap();
                map.entry((i / 4) + 1)
                    .and_modify(|e| e.push_back(c))
                    .or_insert(VecDeque::from([c]));
            }
        }
    }
    map
}

#[derive(Debug, Copy, Clone)]
struct Move {
    from: usize,
    to: usize,
    count: usize,
}

pub struct Problem5_1;

impl Problem5_1 {
    fn parse_line(line: &str) -> Move {
        let mut spl = line.split(' ');
        Move {
            count: spl.nth(1).unwrap().parse::<usize>().unwrap(),
            from: spl.nth(1).unwrap().parse::<usize>().unwrap(),
            to: spl.nth(1).unwrap().parse::<usize>().unwrap(),
        }
    }
}

impl Problem for Problem5_1 {
    fn solve(&self, file_dir: &str) {
        let f = format!("{file_dir}/5_1.txt");
        let mut maps = find_queues(&f);

        let file = File::open(f).unwrap();
        let rdr = BufReader::new(file);

        for mv in rdr
            .lines()
            .filter_map(|ln| ln.ok())
            .filter(|ln| ln.contains("move"))
            .map(|ln| Self::parse_line(&ln))
        {
            for _ in 0..mv.count {
                let v = maps.get_mut(&mv.from).unwrap().pop_front().unwrap();
                maps.get_mut(&mv.to).unwrap().push_front(v);
            }
        }

        let mut keys: Vec<usize> = maps.keys().copied().collect();
        keys.sort();
        let mut res = String::new();
        for k in keys.iter() {
            res.push(*maps.get(k).unwrap().front().unwrap());
        }
        println!("5-1: {res}");
    }
}

pub struct Problem5_2;
impl Problem for Problem5_2 {
    fn solve(&self, file_dir: &str) {
        let f = format!("{file_dir}/5_1.txt");
        let mut maps = find_queues(&f);

        let file = File::open(f).unwrap();
        let rdr = BufReader::new(file);

        let mut q = Vec::with_capacity(20);
        for mv in rdr
            .lines()
            .filter_map(|ln| ln.ok())
            .filter(|ln| ln.contains("move"))
            .map(|ln| Problem5_1::parse_line(&ln))
        {
            for _ in 0..mv.count {
                let v = maps.get_mut(&mv.from).unwrap().pop_front().unwrap();
                q.push(v);
            }

            for c in q.iter().copied().rev() {
                maps.get_mut(&mv.to).unwrap().push_front(c);
            }

            q.clear()
        }

        let mut keys: Vec<usize> = maps.keys().copied().collect();
        keys.sort();
        let mut res = String::new();
        for k in keys.iter() {
            res.push(*maps.get(k).unwrap().front().unwrap());
        }
        println!("5-2: {res}");
    }
}
