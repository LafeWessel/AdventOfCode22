use std::{fs::File, io::{BufReader, BufRead, Write}, ops::Add, hash::Hash, collections::{BinaryHeap, HashMap}};
use petgraph::{prelude::*, dot::Dot, algo::{astar, Measure}, visit::{Visitable, IntoEdges} };
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use super::Problem;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Idx {
    x: usize,
    y: usize,
}


type Map = GraphMap<Idx, u32, Directed>;

pub struct Problem12_1;

impl Problem12_1 {
    pub fn create_graph(lines: &[Vec<char>]) -> (Map, Idx, Idx) {
        let mut map = GraphMap::new();
        let mut start = Idx{
            x: 0, y:0
        };
        let mut end = Idx{x: 0, y: 0};

        // create vertices 
        for (i, ln) in lines.iter().enumerate(){
            for (j, c) in ln.iter().enumerate() {
                let curr = Idx {
                    x: i, y: j 
                } ;
                if *c == 'E' {
                    end = curr;
                }
                if *c == 'S' {
                    start = curr;
                }
                map.add_node(curr);
            }
        }

        // create edges
        let hgt = lines.len();
        let wid = lines[0].len();
        for i in 0..hgt {
            for j in 0..wid {
                let chr = if lines[i][j] == 'S' || lines[i][j] == 'E' {
                    255
                } else {
                    lines[i][j] as u8
                };
                // above
                if i > 0 && ( chr >= Self::convert_char(lines[i-1][j]) as u8 - 1){
                    map.add_edge(Idx {x: i, y: j }, Idx{x: i-1, y: j}, 1);
                }
                // below
                if i + 1 < hgt && (chr >= Self::convert_char(lines[i+1][j]) as u8 - 1){
                    map.add_edge(Idx {x: i, y: j }, Idx{x: i+1, y: j}, 1);
                }
                // left
                if j > 0 && ( chr >= Self::convert_char(lines[i][j-1]) as u8 - 1){
                    map.add_edge(Idx {x: i, y: j }, Idx{x: i, y: j-1}, 1);
                }
                // right
                if j + 1 < wid && ( chr >= Self::convert_char(lines[i][j+1]) as u8 - 1){
                    map.add_edge(Idx {x: i, y: j }, Idx{x: i, y: j+1}, 1);
                }
            }
        }

        (map, start, end)
    }

    fn convert_char(ch: char) -> char {
        if ch == 'S'{
            'a'
        } else if ch == 'E'{
            'z'
        }
        else {
            ch
        }
    }
}

impl Problem for Problem12_1 {
    fn solve(&self, file_dir: &str) {

        let file = File::open(&format!("{file_dir}/12_1.txt")).unwrap();
        let rdr = BufReader::new(file);

        let mut lns = vec![];
        for ln in rdr.lines(){
            let ln = ln.unwrap();
            lns.push(ln.chars().collect::<Vec<_>>());
        }
        let (graph, start, end) = Self::create_graph(&lns);

        println!("Start: {start:?}, end: {end:?}");
        
        println!("Nodes: {}, edges: {}", graph.node_count(), graph.edge_count());

        let path = astar(&graph, start, |n| n == end, |e| *e.2, |_| 0).unwrap();
        println!("12-1: {}", path.0);

    }
}

pub struct Problem12_2;

impl Problem for Problem12_2 {
    fn solve(&self, file_dir: &str) {
        let file = File::open(&format!("{file_dir}/12_1.txt")).unwrap();
        let rdr = BufReader::new(file);

        let mut lns = vec![];
        for ln in rdr.lines(){
            let ln = ln.unwrap();
            lns.push(ln.chars().collect::<Vec<_>>());
        }
        let (graph, _, end) = Problem12_1::create_graph(&lns);

        let starting_points = lns.into_iter().enumerate().map(|(i, l)| l.into_iter().enumerate().filter_map(move |(j, c)| {
            if c == 'a' {
                Some(Idx{x: i, y: j})
            } else {
                None
            }
        })).flatten().collect::<Vec<Idx>>();

        println!("End: {end:?}");
        println!("Nodes: {}, edges: {}", graph.node_count(), graph.edge_count());
        println!("Starting points: {}", starting_points.len());

        let res = starting_points.par_iter().filter_map(|s| astar(&graph, *s, |n| n == end, |e| *e.2, |_| 0)).map(|r| r.0).collect::<Vec<u32>>();
        println!("12-2: {:?}", res.iter().min());
    }
}
