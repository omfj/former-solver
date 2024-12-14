use crate::grid::Grid;
use crate::solver::Solver;
use clap::{Parser, Subcommand};

pub mod grid;
pub mod parser;
pub mod shape;
pub mod solver;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Solve a board
    Solve {
        /// The file containing the board
        #[arg(short, long)]
        grid_file: String,
    },
    /// Play a board
    Play {
        /// The file containing the board
        #[arg(short, long)]
        grid_file: String,

        /// Moves to make
        #[arg(short, long)]
        moves_file: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Solve { grid_file }) => {
            let input = std::fs::read_to_string(grid_file)
                .unwrap_or_else(|_| panic!("Could not read file {}", grid_file));

            let grid = parser::parse_grid(&input).unwrap();

            solve(grid)
        }
        Some(Commands::Play {
            grid_file,
            moves_file,
        }) => {
            let mut moves = vec![];

            let grid_input = std::fs::read_to_string(grid_file)
                .unwrap_or_else(|_| panic!("Could not read file {}", grid_file));

            if moves_file.is_some() {
                let moves_input = std::fs::read_to_string(moves_file.as_ref().unwrap())
                    .unwrap_or_else(|_| {
                        panic!("Could not read file {}", moves_file.as_ref().unwrap())
                    });
                moves = parser::parse_moves(&moves_input).unwrap();
            }

            let grid = parser::parse_grid(&grid_input).unwrap();

            play(grid, moves);
        }
        None => {
            eprintln!("No command provided");
            std::process::exit(1);
        }
    }
}

fn solve(grid: Grid) {
    let solver = Solver::new(grid);
    let moves = solver.beam_search();

    match moves {
        Some(moves) => print_moves(&moves),
        None => println!("No solution found"),
    }
}

fn print_moves(moves: &Vec<(usize, usize)>) {
    println!("Moves to solve board:\n");
    for (i, m) in moves.iter().enumerate() {
        println!("{}) {:?}", i + 1, m);
    }
}

fn play(grid: Grid, moves: Vec<(usize, usize)>) {
    let mut grid = grid.clone();

    for (row, col) in moves {
        grid.remove(row, col);
    }

    if grid.is_empty() {
        println!("Congratulations! You won in {} moves!", grid.moves.len());
        return;
    }

    loop {
        println!("{}", grid);

        let mut input = String::new();
        println!("Enter a coordinate in the format 'row,col' or 'q' to quit: ");
        std::io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input == "q" {
            println!("Goodbye!");
            break;
        }

        let parts = input.split(",").collect::<Vec<&str>>();
        let row = parts[0].parse::<usize>().unwrap();
        let col = parts[1].parse::<usize>().unwrap();

        grid.remove(row, col);

        if grid.is_empty() {
            println!("Congratulations! You won in {} moves!", grid.moves.len());
            break;
        }
    }
}
