mod bfs_solver;
mod dfs_solver;
mod utils;

pub struct SolutionStep {
    pub from: usize,
    pub to: usize,
}

pub trait Solver {
    fn new(height: usize, colors: usize, initial_tubes: Vec<u8>) -> Self;
    fn search(&mut self) -> bool;
    fn get_solution(&self) -> Vec<SolutionStep>;
}

pub use bfs_solver::BFSSolver;
pub use dfs_solver::DFSSolver;
