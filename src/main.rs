use clap::{Parser, Subcommand};

mod cli;

use crate::cli::*;

#[derive(Parser)]
#[command(name = "Water Sort Puzzle")]
#[command(version = "0.1.0")]
#[command(author = "xuanyan <xuanyan@xuanyan.ws>")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Solve the Water Sort Puzzle
    Solve(SolverArgs),

    /// Analyze the Water Sort Puzzle
    Analyze(AnalyzerArgs),
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Solve(subcommand) => {
            run_solver(subcommand);
        }
        Commands::Analyze(subcommand) => {
            run_analyzer(subcommand);
        }
    }
}
