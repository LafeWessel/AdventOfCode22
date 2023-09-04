use std::{collections::HashSet, fs::File, io::Read};

use super::Problem;
use itertools::*;

pub struct Problem6_1;

#[inline]
fn compare_four(v1: u8, v2: u8, v3: u8, v4: u8) -> bool {
    v1 == v2 || v1 == v3 || v1 == v4 || v2 == v3 || v2 == v4 || v3 == v4
}

impl Problem for Problem6_1 {
    fn solve(&self, file_dir: &str) {
        let mut file = File::open(&format!("{file_dir}/6_1.txt")).unwrap();
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();

        let mut ct = 4;
        for (v1, v2, v3, v4) in buf.into_iter().tuple_windows() {
            if !compare_four(v1, v2, v3, v4) {
                break;
            }
            ct += 1;
        }

        println!("6-1: {ct}");
    }
}

pub struct Problem6_2;

impl Problem6_2 {
    fn compare_window(slice: &[u8]) -> bool {
        let mut set = HashSet::new();
        for s in slice {
            if !set.insert(s) {
                return false;
            }
        }
        println!("Slice: {slice:?}");
        true
    }
}

impl Problem for Problem6_2 {
    fn solve(&self, file_dir: &str) {
        let mut file = File::open(&format!("{file_dir}/6_1.txt")).unwrap();
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        println!("Buffer len: {}", buf.len());

        let mut ct = 14;
        let sz = 14;
        let mut i = 0;
        while i + sz < buf.len() {
            let vc = Vec::from_iter(buf[i..i + sz].iter().copied());
            if vc.len() < sz {
                panic!("Ran out of input");
            }

            if Self::compare_window(&vc) {
                break;
            }

            ct += 1;
            i += 1;
        }

        println!("6-2: {ct}");
    }
}
