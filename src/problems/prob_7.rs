use std::{
    cell::RefCell,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
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
        if ln.starts_with("$") {
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
        return (curr_dir, false);
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
    }
}

pub struct Problem7_2;

impl Problem for Problem7_2 {
    fn solve(&self, file_dir: &str) {
        todo!()
    }
}
