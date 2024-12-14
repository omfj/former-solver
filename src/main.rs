use crate::grid::Grid;
use crate::solver::Solver;

pub mod grid;
pub mod parser;
pub mod shape;
pub mod solver;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 3 {
        eprintln!("Usage: {} <command> <file>", args[0]);
        eprintln!("Commands:");
        eprintln!("  solve: Solve the given grid");
        eprintln!("  play: Play the given grid");
        std::process::exit(1);
    }

    let command = &args[1];
    let file_name = &args[2];

    let input = std::fs::read_to_string(file_name)
        .unwrap_or_else(|_| panic!("Could not read file {}", file_name));

    let parser = parser::Parser::new(input);
    let mut grid = parser.parse().unwrap();

    match command.as_str() {
        "solve" => solve(&mut grid),
        "play" => play(&mut grid),
        _ => {
            eprintln!("Invalid command: {}", command);
            std::process::exit(1);
        }
    }
}

fn solve(grid: &mut Grid) {
    #[allow(unused_variables)]
    let solver = Solver::new(grid.clone());
    println!("Not implemented yet! :D");
}

fn play(grid: &mut Grid) {
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
