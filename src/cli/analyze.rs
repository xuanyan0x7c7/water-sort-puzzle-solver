use clap::ArgMatches;
use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time;
use water_sort_puzzle_solver::*;

#[derive(Clone)]
struct Stat {
    moves: Option<usize>,
    duration: f64,
}

pub fn run_analyzer(subcommand: &ArgMatches) {
    let colors: usize = subcommand.value_of("colors").unwrap().parse().unwrap();
    let height: usize = subcommand.value_of("height").unwrap().parse().unwrap();
    let tube_count: usize = subcommand
        .value_of("tubes")
        .unwrap_or((colors + 2).to_string().as_str())
        .parse()
        .unwrap();
    let runs = subcommand.value_of("runs").unwrap().parse().unwrap();

    let stats = (0..runs)
        .map(|_| {
            let mut rng = thread_rng();
            let mut tubes = (0..(colors * height))
                .map(|x| (x / height + 1) as u8)
                .collect_vec();
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
        .collect_vec();

    let solvable_moves = stats.iter().filter_map(|s| s.moves).collect_vec();
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
