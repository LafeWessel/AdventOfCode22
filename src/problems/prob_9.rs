use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use super::Problem;

pub struct Problem9_1;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
impl Direction {
    fn from_str(chr: &str) -> Self {
        match chr {
            "L" => Self::Left,
            "R" => Self::Right,
            "U" => Self::Up,
            "D" => Self::Down,
            _ => panic!("Unknown direction"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Move {
    direction: Direction,
    distance: isize,
}

#[derive(Debug, Copy, Clone)]
struct State {
    head_pos: (isize, isize),
    tail_pos: (isize, isize),
}

impl Problem9_1 {
    fn parse_move(ln: &str) -> Move {
        let mut spl = ln.split(' ');
        let dir = Direction::from_str(spl.next().unwrap());
        let dist = spl.next().unwrap().parse().unwrap();

        Move {
            direction: dir,
            distance: dist,
        }
    }

    fn simple_move(direction: Direction, mut pos: (isize, isize)) -> (isize, isize) {
        // move head
        match direction {
            Direction::Left => {
                pos.1 -= 1;
            }
            Direction::Right => {
                pos.1 += 1;
            }
            Direction::Up => {
                pos.0 += 1;
            }
            Direction::Down => {
                pos.0 -= 1;
            }
        };
        pos
    }

    fn complex_move(
        direction: Direction,
        head: (isize, isize),
        mut tail: (isize, isize),
    ) -> (isize, isize) {
        // move tail
        if (head.0 - tail.0).abs() <= 1 && (head.1 - tail.1).abs() <= 1 {
            return tail;
        }

        match direction {
            Direction::Left => {
                tail.1 -= 1;
                // move diagonal
                if (tail.0 - head.0).abs() >= 1 {
                    if head.0 > tail.0 {
                        tail.0 += 1;
                    } else if head.0 < tail.0 {
                        tail.0 -= 1;
                    }
                }
            }
            Direction::Right => {
                tail.1 += 1;
                // move diagonal
                if (head.0 - tail.0).abs() >= 1 {
                    if head.0 > tail.0 {
                        tail.0 += 1;
                    } else if head.0 < tail.0 {
                        tail.0 -= 1;
                    }
                }
            }
            Direction::Up => {
                tail.0 += 1;
                // move diagonal
                if (head.1 - tail.1).abs() >= 1 {
                    if head.1 > tail.1 {
                        tail.1 += 1;
                    } else if head.1 < tail.1 {
                        tail.1 -= 1;
                    }
                }
            }
            Direction::Down => {
                tail.0 -= 1;
                // move diagonal
                if (head.1 - tail.1).abs() >= 1 {
                    if head.1 > tail.1 {
                        tail.1 += 1;
                    } else if head.1 < tail.1 {
                        tail.1 -= 1;
                    }
                }
            }
        }

        tail
    }
}

impl Problem for Problem9_1 {
    fn solve(&self, file_dir: &str) {
        let mut tail_positions: HashSet<(isize, isize)> = HashSet::new();
        let mut state = State {
            head_pos: (0, 0),
            tail_pos: (0, 0),
        };
        tail_positions.insert((0, 0));

        let file = File::open(&format!("{file_dir}/9_1.txt")).unwrap();
        let rdr = BufReader::new(file);

        for n in rdr.lines() {
            let n = n.unwrap();
            let mv = Self::parse_move(&n);

            for _ in 0..mv.distance {
                state.head_pos = Self::simple_move(mv.direction, state.head_pos);
                state.tail_pos = Self::complex_move(mv.direction, state.head_pos, state.tail_pos);
                tail_positions.insert(state.tail_pos);
                println!("{:?} {state:?}", mv.direction);
            }
        }

        println!("9-1: {}", tail_positions.len());
    }
}

// answer: 2327
pub struct Problem9_2;

impl Problem9_2 {
    fn complex_move(
        direction: Direction,
        head: (isize, isize),
        mut tail: (isize, isize),
    ) -> (isize, isize) {
        // move tail
        if (head.0 - tail.0).abs() <= 1 && (head.1 - tail.1).abs() <= 1 {
            return tail;
        }
        println!("x: {}, y: {}", head.1 - tail.1, head.0 - tail.0);

        match direction {
            Direction::Left => {
                tail.1 -= 1;
                // move diagonal
                if (tail.0 - head.0).abs() >= 1 {
                    if head.0 > tail.0 {
                        tail.0 += 1;
                    } else if head.0 < tail.0 {
                        tail.0 -= 1;
                    }
                }
            }
            Direction::Right => {
                tail.1 += 1;
                // move diagonal
                if (head.0 - tail.0).abs() >= 1 {
                    if head.0 > tail.0 {
                        tail.0 += 1;
                    } else if head.0 < tail.0 {
                        tail.0 -= 1;
                    }
                }
            }
            Direction::Up => {
                tail.0 += 1;
                // move diagonal
                if (head.1 - tail.1).abs() >= 1 {
                    if head.1 > tail.1 {
                        tail.1 += 1;
                    } else if head.1 < tail.1 {
                        tail.1 -= 1;
                    }
                }
            }
            Direction::Down => {
                tail.0 -= 1;
                // move diagonal
                if (head.1 - tail.1).abs() >= 1 {
                    if head.1 > tail.1 {
                        tail.1 += 1;
                    } else if head.1 < tail.1 {
                        tail.1 -= 1;
                    }
                }
            }
        }
        // let diff = (head.1 - tail.1, head.0 - tail.0);
        // let (dx, dy) = match diff {
        //     // overlapping
        //     (0, 0) => (0, 0),
        //     // touching up/left/down/right
        //     (0, 1) | (1, 0) | (0, -1) | (-1, 0) => (0, 0),
        //     // touching diagonally
        //     (1, 1) | (1, -1) | (-1, 1) | (-1, -1) => (0, 0),
        //     // need to move up/left/down/right
        //     (0, 2) => (0, 1),
        //     (0, -2) => (0, -1),
        //     (2, 0) => (1, 0),
        //     (-2, 0) => (-1, 0),
        //     // need to move to the right diagonally
        //     (2, 1) => (1, 1),
        //     (2, -1) => (1, -1),
        //     // need to move to the left diagonally
        //     (-2, 1) => (-1, 1),
        //     (-2, -1) => (-1, -1),
        //     // need to move up/down diagonally
        //     (1, 2) => (1, 1),
        //     (-1, 2) => (-1, 1),
        //     (1, -2) => (1, -1),
        //     (-1, -2) => (-1, -1),
        //     // 🆕 need to move diagonally
        //     (-2, -2) => (-1, -1),
        //     (-2, 2) => (-1, 1),
        //     (2, -2) => (1, -1),
        //     (2, 2) => (1, 1),
        //     _ => panic!("unhandled case: tail - head = {diff:?}"),
        // };
        // tail.1 += dx;
        // tail.0 += dy;
        tail
    }
}

impl Problem for Problem9_2 {
    fn solve(&self, file_dir: &str) {
        let mut tail_positions: HashSet<(isize, isize)> = HashSet::new();
        tail_positions.insert((0, 0));
        let mut rope = vec![(0, 0); 10];

        let file = File::open(&format!("{file_dir}/9_1.txt")).unwrap();
        let rdr = BufReader::new(file);

        for n in rdr.lines() {
            let n = n.unwrap();
            let mv = Problem9_1::parse_move(&n);

            for _ in 0..mv.distance {
                rope[0] = Problem9_1::simple_move(mv.direction, rope[0]);
                rope[1] = Problem9_2::complex_move(mv.direction, rope[0], rope[1]);
                rope[2] = Problem9_2::complex_move(mv.direction, rope[1], rope[2]);
                rope[3] = Problem9_2::complex_move(mv.direction, rope[2], rope[3]);
                rope[4] = Problem9_2::complex_move(mv.direction, rope[3], rope[4]);
                rope[5] = Problem9_2::complex_move(mv.direction, rope[4], rope[5]);
                rope[6] = Problem9_2::complex_move(mv.direction, rope[5], rope[6]);
                rope[7] = Problem9_2::complex_move(mv.direction, rope[6], rope[7]);
                rope[8] = Problem9_2::complex_move(mv.direction, rope[7], rope[8]);
                rope[9] = Problem9_2::complex_move(mv.direction, rope[8], rope[9]);
                tail_positions.insert(rope[9]);
                println!("{:?} {rope:?}", mv.direction);
            }
        }

        println!("9-1: {}", tail_positions.len());
    }
}

#[test]
fn test_9_2() {
    let cmds = vec![
        Move {
            direction: Direction::Right,
            distance: 5,
        },
        Move {
            direction: Direction::Up,
            distance: 8,
        },
        Move {
            direction: Direction::Left,
            distance: 8,
        },
        Move {
            direction: Direction::Down,
            distance: 3,
        },
        Move {
            direction: Direction::Right,
            distance: 17,
        },
        Move {
            direction: Direction::Down,
            distance: 10,
        },
        Move {
            direction: Direction::Left,
            distance: 25,
        },
        Move {
            direction: Direction::Up,
            distance: 20,
        },
    ];
    // let cmds = vec![
    //     Move {
    //         direction: Direction::Right,
    //         distance: 4,
    //     },
    //     Move {
    //         direction: Direction::Up,
    //         distance: 4,
    //     },
    //     Move {
    //         direction: Direction::Left,
    //         distance: 3,
    //     },
    //     Move {
    //         direction: Direction::Down,
    //         distance: 1,
    //     },
    //     Move {
    //         direction: Direction::Right,
    //         distance: 4,
    //     },
    //     Move {
    //         direction: Direction::Down,
    //         distance: 1,
    //     },
    //     Move {
    //         direction: Direction::Left,
    //         distance: 5,
    //     },
    //     Move {
    //         direction: Direction::Right,
    //         distance: 2,
    //     },
    // ];
    let mut tail_positions: HashSet<(isize, isize)> = HashSet::new();
    tail_positions.insert((0, 0));
    let mut rope = vec![(25, 25); 10];

    for mv in cmds {
        for _ in 0..mv.distance {
            let mut grid: Vec<Vec<u8>> = vec![vec![255; 50]; 50];
            rope[0] = Problem9_1::simple_move(mv.direction, rope[0]);
            rope[1] = Problem9_2::complex_move(mv.direction, rope[0], rope[1]);
            rope[2] = Problem9_2::complex_move(mv.direction, rope[1], rope[2]);
            rope[3] = Problem9_2::complex_move(mv.direction, rope[2], rope[3]);
            rope[4] = Problem9_2::complex_move(mv.direction, rope[3], rope[4]);
            rope[5] = Problem9_2::complex_move(mv.direction, rope[4], rope[5]);
            rope[6] = Problem9_2::complex_move(mv.direction, rope[5], rope[6]);
            rope[7] = Problem9_2::complex_move(mv.direction, rope[6], rope[7]);
            rope[8] = Problem9_2::complex_move(mv.direction, rope[7], rope[8]);
            rope[9] = Problem9_2::complex_move(mv.direction, rope[8], rope[9]);
            tail_positions.insert(rope[9]);
            println!("{:?} {rope:?}", mv);
            for (i, r) in rope.iter().enumerate().rev() {
                grid[r.0 as usize][r.1 as usize] = i as u8;
            }

            //print board
            for i in grid.iter().rev() {
                for j in i.iter() {
                    let mut chr = format!("{j}");
                    if *j == 255 {
                        chr = format!(".");
                    }
                    print!("{chr} ");
                }
                println!("");
            }
        }
    }
    let mut grid: Vec<Vec<u8>> = vec![vec![255; 50]; 50];
    for p in tail_positions.iter() {
        grid[p.0 as usize][p.1 as usize] = 0;
    }
    for i in grid.iter().rev() {
        for j in i.iter() {
            let mut chr = format!("{j}");
            if *j == 255 {
                chr = format!(".");
            }
            print!("{chr} ");
        }
        println!("");
    }
}
