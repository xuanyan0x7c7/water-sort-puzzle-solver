use clap::ArgMatches;
use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::io::stdin;
use std::process;
use std::time;
use water_sort_puzzle_solver::*;

fn solve(solver: &mut impl Solver) {
    let now = time::Instant::now();
    if solver.search() {
        for step in solver.get_solution() {
            println!("{} -> {}", step.from + 1, step.to + 1);
        }
    } else {
        println!("No solution.");
    }
    println!("Time used: {} seconds", now.elapsed().as_secs_f64());
}

pub fn run_solver(subcommand: &ArgMatches) {
    let color_count: usize = subcommand.value_of("colors").unwrap().parse().unwrap();
    let height: usize = subcommand.value_of("height").unwrap().parse().unwrap();
    let tube_count: usize = subcommand
        .value_of("tubes")
        .unwrap_or((color_count + 2).to_string().as_str())
        .parse()
        .unwrap();
    let use_dfs = subcommand.is_present("suboptimal");

    let mut color_list = vec![];
    for _ in 0..tube_count {
        let mut line_input = String::new();
        match stdin().read_line(&mut line_input) {
            Ok(_) => {
                color_list.push(
                    line_input
                        .split_ascii_whitespace()
                        .take(height)
                        .map(String::from)
                        .collect_vec(),
                );
            }
            Err(error) => {
                eprintln!("Error: {error}");
                process::exit(1);
            }
        }
    }
    let mut color_map = FxHashMap::<String, (usize, usize)>::default();
    for colors in color_list.iter() {
        for c in colors {
            if let Some(item) = color_map.get_mut(c) {
                item.1 += 1;
            } else {
                color_map.insert(c.to_string(), (color_map.len(), 1));
            }
        }
    }
    if color_map.len() != color_count {
        eprintln!(
            "Number of colors mismatch: expected {color_count}, actual {}",
            color_map.len()
        );
        process::exit(1);
    }
    for (color, &(_, count)) in color_map.iter() {
        if count != height {
            eprintln!("Color {color} count mismatch: expected {height}, actual {count}");
            process::exit(1);
        }
    }
    let mut tubes = vec![0; tube_count * height];
    for (tube_index, colors) in color_list.iter().enumerate() {
        for (index, c) in colors.iter().enumerate() {
            tubes[tube_index * height + index] = (color_map.get(c).unwrap().0 + 1) as u8;
        }
    }

    if use_dfs {
        solve(&mut DFSSolver::new(height, tubes));
    } else {
        solve(&mut BFSSolver::new(height, tubes));
    }
}
