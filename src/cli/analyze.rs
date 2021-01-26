use clap::ArgMatches;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time;
use water_sort_puzzle_solver::{BFSSolver, Solver, Tube};

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
    let runs: usize = subcommand.value_of("runs").unwrap().parse().unwrap();

    let mut stats: Vec<Stat> = vec![];

    for _ in 0..runs {
        let mut rng = thread_rng();
        let mut pattern: Vec<u8> = (0..(colors * height)).map(|x| (x / height) as u8).collect();
        pattern.shuffle(&mut rng);
        let mut tubes: Vec<Tube> = vec![];
        for color in 0..colors {
            tubes.push(pattern[(color * height)..((color + 1) * height)].to_vec());
        }
        for _ in 0..(tube_count - colors) {
            tubes.push(vec![]);
        }
        let mut solver = BFSSolver::new(height, colors, &tubes);
        let now = time::Instant::now();
        let mut stat = Stat {
            moves: None,
            duration: 0.0,
        };
        if solver.search() {
            stat.moves = Some(solver.get_solution().len());
        }
        stat.duration = now.elapsed().as_secs_f64();
        stats.push(stat);
    }

    let solvable_stats: Vec<Stat> = stats
        .iter()
        .filter(|s| s.moves.is_some())
        .map(|s| s.clone())
        .collect();
    let solvable_count = solvable_stats.len();
    let total_moves = solvable_stats
        .iter()
        .map(|s| s.moves.unwrap())
        .sum::<usize>();
    let total_square_moves = solvable_stats
        .iter()
        .map(|s| s.moves.unwrap().pow(2))
        .sum::<usize>();
    let min_moves = solvable_stats
        .iter()
        .map(|s| s.moves.unwrap())
        .min()
        .unwrap();
    let max_moves = solvable_stats
        .iter()
        .map(|s| s.moves.unwrap())
        .max()
        .unwrap();
    let total_time = stats.iter().map(|s| s.duration).sum::<f64>();
    let total_square_time = stats.iter().map(|s| s.duration.powi(2)).sum::<f64>();
    let min_time = stats
        .iter()
        .map(|s| s.duration)
        .min_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap();
    let max_time = stats
        .iter()
        .map(|s| s.duration)
        .max_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap();

    println!("{}% solvable.", (solvable_count * 100) as f64 / runs as f64);
    println!(
        "Average {} moves, min {}, max {}, stddev {}.",
        total_moves as f64 / solvable_count as f64,
        min_moves,
        max_moves,
        (total_square_moves as f64 / solvable_count as f64
            - (total_moves as f64 / solvable_count as f64).powi(2))
        .sqrt(),
    );
    println!(
        "Average time {}, min {}, max {}, stddev {}.",
        total_time as f64 / solvable_count as f64,
        min_time,
        max_time,
        (total_square_time as f64 / solvable_count as f64
            - (total_time as f64 / solvable_count as f64).powi(2))
        .sqrt(),
    );
}
