use clap::{Parser, ValueEnum};
use problems::{
    prob_1::{Problem1_1, Problem1_2},
    prob_10::{Problem10_1, Problem10_2},
    prob_2::{Problem2_1, Problem2_2},
    prob_3::{Problem3_1, Problem3_2},
    prob_4::{Problem4_1, Problem4_2},
    prob_5::{Problem5_1, Problem5_2},
    prob_6::{Problem6_1, Problem6_2},
    prob_7::{Problem7_1, Problem7_2},
    prob_8::{Problem8_1, Problem8_2},
    prob_9::{Problem9_1, Problem9_2},
    Problem, prob_11::{Problem11_1, Problem11_2},
};

mod problems;

#[derive(ValueEnum, Clone, Debug)]
enum Problems {
    P11,
    P12,
    P21,
    P22,
    P31,
    P32,
    P41,
    P42,
    P51,
    P52,
    P61,
    P62,
    P71,
    P72,
    P81,
    P82,
    P91,
    P92,
    P101,
    P102,
    P111,
    P112,
}

impl Problems {
    fn problem(self) -> Box<dyn Problem> {
        match self {
            Problems::P11 => Box::new(Problem1_1),
            Problems::P12 => Box::new(Problem1_2),
            Problems::P21 => Box::new(Problem2_1),
            Problems::P22 => Box::new(Problem2_2),
            Problems::P31 => Box::new(Problem3_1),
            Problems::P32 => Box::new(Problem3_2),
            Problems::P41 => Box::new(Problem4_1),
            Problems::P42 => Box::new(Problem4_2),
            Problems::P51 => Box::new(Problem5_1),
            Problems::P52 => Box::new(Problem5_2),
            Problems::P61 => Box::new(Problem6_1),
            Problems::P62 => Box::new(Problem6_2),
            Problems::P71 => Box::new(Problem7_1),
            Problems::P72 => Box::new(Problem7_2),
            Problems::P81 => Box::new(Problem8_1),
            Problems::P82 => Box::new(Problem8_2),
            Problems::P91 => Box::new(Problem9_1),
            Problems::P92 => Box::new(Problem9_2),
            Problems::P101 => Box::new(Problem10_1),
            Problems::P102 => Box::new(Problem10_2),
            Problems::P111 => Box::new(Problem11_1),
            Problems::P112 => {
                Box::new(Problem11_2)
            },
        }
    }
}

#[derive(Parser)]
struct Args {
    #[arg(long, short, help = "Problem to run")]
    problem: Problems,
    #[arg(long, short, help = "File dir to use", default_value_t = String::from("./data"))]
    file_dir: String,
}

fn main() {
    let args = Args::parse();

    args.problem.problem().solve(&args.file_dir)
}
