use std::time;

use clap::Args;
use itertools::Itertools;
use rand::rng;
use rand::seq::SliceRandom;

use water_sort_puzzle_solver::*;

#[derive(Clone)]
struct Stat {
    moves: Option<usize>,
    duration: f64,
}

#[derive(Args)]
pub struct AnalyzerArgs {
    /// Number of colors in the puzzle.
    #[arg(short, long, value_parser)]
    colors: usize,

    /// Height of each tube.
    #[arg(short = 'H', long, value_parser)]
    height: usize,

    /// Number of tubes. Defaults to `colors + 2`.
    #[arg(short, long, value_parser)]
    tubes: Option<usize>,

    /// Number of runs to perform.
    #[arg(short = 'n', long, value_parser, default_value_t = 1000)]
    runs: usize,
}

pub fn run_analyzer(subcommand: &AnalyzerArgs) {
    let colors: usize = subcommand.colors;
    let height: usize = subcommand.height;
    let tube_count: usize = subcommand.tubes.unwrap_or(colors + 2);
    let runs = subcommand.runs;

    let stats: Vec<Stat> = (0..runs)
        .map(|_| {
            let mut rng = rng();
            let mut tubes: Vec<u8> = (0..(colors * height))
                .map(|x| (x / height + 1) as u8)
                .collect();
            tubes.shuffle(&mut rng);
            tubes.resize(tube_count * height, 0);
            let mut solver = BFSSolver::new(height, tubes);
            let now = time::Instant::now();
            let moves = if solver.search() {
                Some(solver.get_solution().len())
            } else {
                None
            };
            let duration = now.elapsed().as_secs_f64();
            Stat { moves, duration }
        })
        .collect();

    let solvable_moves: Vec<usize> = stats.iter().filter_map(|s| s.moves).collect();
    let solvable_count = solvable_moves.len();

    if solvable_count == 0 {
        println!("0% solvable.");
        return;
    }

    let total_moves = solvable_moves.iter().sum::<usize>();
    let total_square_moves = solvable_moves
        .iter()
        .map(|moves| moves.pow(2))
        .sum::<usize>();
    let (&min_moves, &max_moves) = solvable_moves.iter().minmax().into_option().unwrap();
    let total_time = stats.iter().map(|s| s.duration).sum::<f64>();
    let total_square_time = stats.iter().map(|s| s.duration.powi(2)).sum::<f64>();
    let (min_time, max_time) = stats
        .iter()
        .map(|s| s.duration)
        .minmax_by(|x, y| x.partial_cmp(y).unwrap())
        .into_option()
        .unwrap();

    println!("{}% solvable.", (solvable_count * 100) as f64 / runs as f64);
    println!(
        "Average {} moves, min {min_moves}, max {max_moves}, stddev {}.",
        total_moves as f64 / solvable_count as f64,
        (total_square_moves as f64 / solvable_count as f64
            - (total_moves as f64 / solvable_count as f64).powi(2))
        .sqrt(),
    );
    println!(
        "Average time {}, min {min_time}, max {max_time}, stddev {}.",
        total_time / solvable_count as f64,
        (total_square_time / solvable_count as f64 - (total_time / solvable_count as f64).powi(2))
            .sqrt(),
    );
}
