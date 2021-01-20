mod bfs_solver;
mod dfs_solver;
mod solver;

use crate::bfs_solver::BFSSolver;
use crate::dfs_solver::DFSSolver;
use crate::solver::{Solver, Tube};
use std::collections::HashMap;
use std::env;
use std::io::stdin;
use std::process;
use std::time;

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

fn main() {
    let mut color_count = 0usize;
    let mut height = 0usize;
    let mut tube_count = 0usize;
    let mut use_dfs = false;

    for arg in env::args().skip(1) {
        let list: Vec<&str> = arg.split('=').collect();
        if list.len() == 1 {
            if list[0] == "--suboptimal" {
                use_dfs = true;
            }
        } else if list.len() == 2 {
            if list[0] == "--colors" {
                color_count = list[1].parse().unwrap_or(0);
            } else if list[0] == "--height" {
                height = list[1].parse().unwrap_or(0);
            } else if list[0] == "--tubes" {
                tube_count = list[1].parse().unwrap_or(0);
            }
        }
    }

    if color_count == 0 {
        eprintln!("Error: Colors not specified");
        process::exit(1);
    }
    if height == 0 {
        eprintln!("Error: Height not specified");
        process::exit(1);
    }
    if tube_count == 0 {
        eprintln!("Error: Tubes not specified");
        process::exit(1);
    }

    let mut color_list: Vec<Vec<String>> = vec![];
    for _ in 0..tube_count {
        let mut line_input = String::new();
        match stdin().read_line(&mut line_input) {
            Ok(_) => {
                color_list.push(
                    line_input
                        .split_ascii_whitespace()
                        .take(height)
                        .map(|s| String::from(s))
                        .collect(),
                );
            }
            Err(error) => {
                eprintln!("Error: {}", error);
                process::exit(1);
            }
        }
    }
    let mut color_map: HashMap<String, (usize, usize)> = HashMap::new();
    for colors in color_list.iter() {
        for c in colors {
            match color_map.get_mut(c) {
                Some(item) => {
                    item.1 += 1;
                }
                None => {
                    color_map.insert(c.to_string(), (color_map.len(), 1));
                }
            }
        }
    }
    if color_map.len() != color_count {
        eprintln!(
            "Number of colors mismatch: expected {}, actual {}",
            color_count,
            color_map.len()
        );
        process::exit(1);
    }
    for (color, (_, count)) in color_map.iter() {
        if *count != height {
            eprintln!(
                "Color {} count mismatch: expected {}, actual {}",
                color, height, count,
            );
            process::exit(1);
        }
    }
    let tubes: Vec<Tube> = color_list
        .iter()
        .map(|colors| {
            colors
                .iter()
                .map(|c| color_map.get(c).unwrap().0 as u8)
                .collect()
        })
        .collect();

    if use_dfs {
        solve(&mut DFSSolver::new(height, color_count, &tubes));
    } else {
        solve(&mut BFSSolver::new(height, color_count, &tubes));
    }
}
