use std::{
    cell::RefCell,
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
};

use super::Problem;

#[derive(Clone, Debug)]
struct NodeDir {
    parent: Option<Rc<RefCell<NodeDir>>>,
    children: Vec<Rc<RefCell<NodeDir>>>,
    files: Vec<NodeFile>,
    name: String,
}

impl NodeDir {
    fn size(&self) -> usize {
        let mut sz = 0;
        for c in self.children.iter() {
            sz += c.borrow().size();
        }
        sz += self.files.iter().map(|f| f.size).sum::<usize>();
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
    fn parse_line(ln: &str, curr_dir: Rc<RefCell<NodeDir>>, reading_output: bool) -> bool {
        // reading command
        if ln.starts_with("$") {
            if ln.starts_with("$ cd") {
                let dir = ln.replace("$ cd ", "");

                // go until there are no parents
                if dir == "/" {
                    while let Some(ref d) = curr_dir.to_owned().borrow().parent {
                        curr_dir.swap(d);
                    }
                } else if dir == ".." {
                    let par = &curr_dir.borrow().parent;
                    let par = par.as_ref().unwrap();
                    println!(
                        "Parent: {}, current: {}",
                        par.borrow().name,
                        curr_dir.borrow().name
                    );
                    curr_dir.try_borrow().expect("Could not borrow current");
                    par.try_borrow().expect("Could not borrow parent");
                    curr_dir.swap(&par);
                } else {
                    println!(
                        "Current dir: {}, current parent: {:?}",
                        curr_dir.borrow().name,
                        curr_dir
                            .borrow()
                            .parent
                            .clone()
                            .map(|f| f.borrow().clone().name)
                    );
                    let mut n = Rc::clone(&curr_dir);
                    let mut par = None;
                    for c in curr_dir.to_owned().borrow().children.iter() {
                        if c.borrow().name == dir {
                            n = Rc::clone(c);
                            par = c.borrow().clone().parent;
                        }
                    }
                    println!(
                        "New dir: {}, new parent: {:?}",
                        n.borrow().name,
                        n.borrow().parent.clone().map(|f| f.borrow().clone().name)
                    );

                    curr_dir.swap(&n);
                    curr_dir.borrow_mut().parent = par;
                    println!(
                        "New dir: {}, new parent: {:?}",
                        curr_dir.borrow().name,
                        curr_dir
                            .borrow()
                            .parent
                            .clone()
                            .map(|f| f.borrow().clone().name)
                    );
                }
                return false;
            } else if ln.starts_with("$ ls") {
                return true;
            }
        } else if reading_output {
            if ln.starts_with("dir") {
                let dirname = ln.split(' ').nth(1).unwrap().to_owned();
                curr_dir
                    .borrow_mut()
                    .children
                    .push(Rc::new(RefCell::new(NodeDir {
                        children: vec![],
                        files: vec![],
                        name: dirname,
                        parent: Some(Rc::clone(&curr_dir)),
                    })));
                let names = curr_dir
                    .borrow()
                    .children
                    .iter()
                    .last()
                    .map(|f| f.borrow().clone().name);
                let par = curr_dir
                    .borrow()
                    .children
                    .iter()
                    .last()
                    .map(|f| f.borrow().clone().parent.unwrap().borrow().clone().name);
                println!(
                    "New child: {:?}, child's parent: {:?}, current parent: {:?}",
                    names,
                    par,
                    curr_dir
                        .borrow()
                        .parent
                        .clone()
                        .map(|p| p.borrow().clone().name)
                );
            } else {
                let mut spl = ln.split(' ');
                let sz = spl.next().unwrap().parse::<usize>().unwrap();
                let nm = spl.next().unwrap();
                curr_dir.borrow_mut().files.push(NodeFile {
                    name: nm.to_owned(),
                    size: sz,
                });
                println!("New files: {:?}", curr_dir.borrow().files);
            }
            return true;
        } else {
            panic!("Doing nothing!");
        }
        return false;
    }
}

impl Problem for Problem7_1 {
    fn solve(&self, file_dir: &str) {
        let file = File::open(&format!("{file_dir}/7_1.txt")).unwrap();
        let rdr = BufReader::new(file);

        let root = Rc::new(RefCell::new(NodeDir {
            children: vec![],
            files: vec![],
            name: "/".to_owned(),
            parent: None,
        }));
        let mut sts = false;
        for (i, ln) in rdr.lines().enumerate() {
            let ln = ln.unwrap();
            println!("{i}: Current dir: {}: command: {ln}", root.borrow().name,);
            sts = Self::parse_line(&ln, Rc::clone(&root), sts);
        }
    }
}

pub struct Problem7_2;

impl Problem for Problem7_2 {
    fn solve(&self, file_dir: &str) {
        todo!()
    }
}
