use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use super::Problem;

pub struct Problem8_1;

impl Problem8_1 {
    fn is_hidden(x: usize, y: usize, grid: &Vec<Vec<u8>>) -> bool {
        // on edge
        if y == 0 || x == 0 || x + 1 == grid[x].len() || y + 1 == grid.len() {
            return false;
        }
        let val = grid[x][y];
        if val == 0 {
            return true;
        }
        // above
        let mut top = false;
        for i in 0..y {
            if grid[x][i] >= val {
                top = true;
                break;
            }
        }
        // below
        let mut bottom = false;
        for i in y + 1..grid[x].len() {
            if grid[x][i] >= val {
                bottom = true;
                break;
            }
        }
        // left
        let mut left = false;
        for i in 0..x {
            if grid[i][y] >= val {
                left = true;
                break;
            }
        }
        // right
        let mut right = false;
        for i in x + 1..grid.len() {
            if grid[i][y] >= val {
                right = true;
                break;
            }
        }

        left && bottom && right && top
    }
}

impl Problem for Problem8_1 {
    fn solve(&self, file_dir: &str) {
        let file = File::open(&format!("{file_dir}/8_1.txt")).unwrap();
        let rdr = BufReader::new(file);

        let mut grid: Vec<Vec<u8>> = vec![];

        for r in rdr.lines() {
            grid.push(
                r.unwrap()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect(),
            );
        }
        println!(
            "Grid: {} x {}; total: {}",
            grid.len(),
            grid[0].len(),
            grid.iter().map(|g| g.len()).sum::<usize>()
        );

        let mut visible = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                let hid = Self::is_hidden(i, j, &grid);
                println!("x: {i}, y: {j}, val: {}, hidden: {hid}", grid[i][j]);
                if !hid {
                    visible += 1;
                }
            }
        }

        println!("8-1: {visible}");
    }
}

pub struct Problem8_2;

impl Problem8_2 {
    fn tree_count(x: usize, y: usize, grid: &Vec<Vec<u8>>) -> usize {
        let val = grid[x][y];
        // above
        let mut top = 0;
        for i in (0..y).rev() {
            if grid[x][i] >= val {
                top += 1;
                break;
            } else {
                top += 1;
            }
        }
        // below
        let mut bottom = 0;
        for i in y + 1..grid[x].len() {
            if grid[x][i] >= val {
                bottom += 1;
                break;
            } else {
                bottom += 1;
            }
        }
        // left
        let mut left = 0;
        for i in (0..x).rev() {
            if grid[i][y] >= val {
                left += 1;
                break;
            } else {
                left += 1;
            }
        }
        // right
        let mut right = 0;
        for i in x + 1..grid.len() {
            if grid[i][y] >= val {
                right += 1;
                break;
            } else {
                right += 1;
            }
        }

        println!("left: {left}, right: {right}, top: {top}, bottom: {bottom}");
        left * bottom * right * top
    }
}

impl Problem for Problem8_2 {
    fn solve(&self, file_dir: &str) {
        let file = File::open(&format!("{file_dir}/8_1.txt")).unwrap();
        let rdr = BufReader::new(file);

        let mut grid: Vec<Vec<u8>> = vec![];

        for r in rdr.lines() {
            grid.push(
                r.unwrap()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect(),
            );
        }
        println!(
            "Grid: {} x {}; total: {}",
            grid.len(),
            grid[0].len(),
            grid.iter().map(|g| g.len()).sum::<usize>()
        );

        let mut max = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                let ct = Self::tree_count(i, j, &grid);
                println!("x: {i}, y: {j}, val: {}, count: {ct}", grid[i][j]);
                if ct > max {
                    max = ct;
                }
            }
        }

        println!("8-1: {max}");
    }
}
