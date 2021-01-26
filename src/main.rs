mod cli;

use crate::cli::analyze::run_analyzer;
use crate::cli::solve::run_solver;
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("Water Sort Puzzle")
        .version("0.1.0")
        .author("xuanyan <xuanyan@xuanyan.ws>")
        .subcommand(
            SubCommand::with_name("solve")
                .about("Solve Puzzle")
                .arg(
                    Arg::with_name("colors")
                        .short("c")
                        .long("colors")
                        .help("Colors")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("height")
                        .short("H")
                        .long("height")
                        .help("Tube height")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("tubes")
                        .short("t")
                        .long("tubes")
                        .help("Number of tubes")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("suboptimal")
                        .long("suboptimal")
                        .help("Find suboptimal solution"),
                ),
        )
        .subcommand(
            SubCommand::with_name("analyze")
                .about("Analyze Puzzle")
                .arg(
                    Arg::with_name("colors")
                        .short("c")
                        .long("colors")
                        .help("Colors")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("height")
                        .short("H")
                        .long("height")
                        .help("Tube height")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("tubes")
                        .short("t")
                        .long("tubes")
                        .help("Number of tubes")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("runs")
                        .short("n")
                        .long("runs")
                        .help("Number of runs")
                        .takes_value(true)
                        .default_value("1000"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("solve", Some(subcommand)) => {
            run_solver(subcommand);
        }
        ("analyze", Some(subcommand)) => {
            run_analyzer(subcommand);
        }
        _ => {}
    }
}
