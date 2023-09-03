use clap::{Parser, ValueEnum};
use problems::{
    prob_1::{Problem1_1, Problem1_2},
    prob_2::{Problem2_1, Problem2_2},
    prob_3::{Problem3_1, Problem3_2},
    prob_4::{Problem4_1, Problem4_2},
    Problem,
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
