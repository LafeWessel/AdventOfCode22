pub mod prob_1;
pub mod prob_2;
pub mod prob_3;
pub mod prob_4;
pub mod prob_5;
pub mod prob_6;
pub mod prob_7;

pub trait Problem {
    fn solve(&self, file_dir: &str);
}
