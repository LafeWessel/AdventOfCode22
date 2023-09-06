use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};
use uuid::Uuid;

use super::Problem;

#[derive(Clone, Debug)]
struct Dir {
    id: Uuid,
    parent: Option<Uuid>,
    children: Vec<Uuid>,
    files: Vec<NodeFile>,
    name: String,
}

impl Dir {
    fn size(&self, map: &HashMap<Uuid, Dir>) -> usize {
        let mut sz = 0;
        sz += map
            .get(&self.id)
            .unwrap()
            .files
            .iter()
            .map(|f| f.size)
            .sum::<usize>();
        for c in map.get(&self.id).unwrap().children.iter() {
            sz += map.get(c).unwrap().size(map);
        }
        sz
    }
}

#[derive(Clone, Debug)]
struct NodeFile {
    name: String,
    size: usize,
}

pub struct Problem7_1;

impl Problem7_1 {
    fn parse_line(
        ln: &str,
        graph: &mut HashMap<Uuid, Dir>,
        curr_dir: Uuid,
        reading_output: bool,
    ) -> (Uuid, bool) {
        // reading command
        if ln.starts_with('$') {
            if ln.starts_with("$ cd") {
                let dir = ln.replace("$ cd ", "");

                let mut next = curr_dir;
                if dir == "/" {
                    while let Some(n) = graph.get(&next).unwrap().parent {
                        next = n;
                    }
                } else if dir == ".." {
                    next = graph.get(&curr_dir).unwrap().parent.unwrap();
                } else {
                    for v in graph.get(&curr_dir).unwrap().children.iter() {
                        if graph.get(v).unwrap().name == dir {
                            next = *v;
                        }
                    }
                }
                return (next, false);
            } else if ln.starts_with("$ ls") {
                return (curr_dir, true);
            }
        } else if reading_output {
            if ln.starts_with("dir") {
                let dirname = ln.split(' ').nth(1).unwrap().to_owned();
                let new_dir = Dir {
                    id: Uuid::new_v4(),
                    parent: Some(curr_dir),
                    children: vec![],
                    files: vec![],
                    name: dirname,
                };
                graph.get_mut(&curr_dir).unwrap().children.push(new_dir.id);
                graph.insert(new_dir.id, new_dir);
            } else {
                let mut spl = ln.split(' ');
                let sz = spl.next().unwrap().parse::<usize>().unwrap();
                let nm = spl.next().unwrap();
                graph.get_mut(&curr_dir).unwrap().files.push(NodeFile {
                    name: nm.to_owned(),
                    size: sz,
                });
            }
            return (curr_dir, true);
        } else {
            panic!("Doing nothing!");
        }
        (curr_dir, false)
    }

    fn calculate_sizes(map: &HashMap<Uuid, Dir>) -> HashMap<Uuid, usize> {
        let mut sizes = HashMap::new();

        for (k, v) in map.iter() {
            sizes.insert(*k, v.size(map));
        }

        sizes
    }
}

impl Problem for Problem7_1 {
    fn solve(&self, file_dir: &str) {
        let file = File::open(&format!("{file_dir}/7_1.txt")).unwrap();
        let rdr = BufReader::new(file);

        let root = Dir {
            children: vec![],
            files: vec![],
            name: "/".to_owned(),
            parent: None,
            id: Uuid::new_v4(),
        };
        let mut next = root.id;
        let mut map = HashMap::from([(root.id, root.clone())]);
        let mut sts = false;
        for (i, ln) in rdr.lines().enumerate() {
            let ln = ln.unwrap();
            println!("{i} - {}: {ln}", root.name,);
            (next, sts) = Self::parse_line(&ln, &mut map, next, sts);
        }
        let sizes = Self::calculate_sizes(&map);

        let overall_sz = sizes.values().filter(|f| **f < 100_000).sum::<usize>();
        println!("7-1: {overall_sz}");
    }
}

pub struct Problem7_2;

impl Problem for Problem7_2 {
    fn solve(&self, file_dir: &str) {
        let file = File::open(&format!("{file_dir}/7_1.txt")).unwrap();
        let rdr = BufReader::new(file);

        let root = Dir {
            children: vec![],
            files: vec![],
            name: "/".to_owned(),
            parent: None,
            id: Uuid::new_v4(),
        };
        let mut next = root.id;
        let mut map = HashMap::from([(root.id, root.clone())]);
        let mut sts = false;
        for (i, ln) in rdr.lines().enumerate() {
            let ln = ln.unwrap();
            println!("{i} - {}: {ln}", root.name,);
            (next, sts) = Problem7_1::parse_line(&ln, &mut map, next, sts);
        }
        let sizes = Problem7_1::calculate_sizes(&map);

        let overall_sz = sizes.values().max().unwrap();

        let overall_space = 70_000_000;
        let required_space = 30_000_000;
        println!("Space left: {}", overall_space - overall_sz);
        let needed_space = required_space - (overall_space - overall_sz);
        println!("Needed space: {}", needed_space);

        let dir = sizes
            .iter()
            .filter(|(_f, s)| **s >= needed_space)
            .min_by(|(_f1, s1), (_f2, s2)| s1.cmp(s2))
            .unwrap();

        println!("7-1: {dir:?}");
    }
}
