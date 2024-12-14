use crate::grid::Grid;
use crate::shape::{Color, Shape};

pub fn parse_grid(input: &str) -> Result<Grid, String> {
    let mut grid = Grid::new(vec![]);

    for (i, line) in input.lines().enumerate() {
        let mut row = vec![];

        for (j, c) in line.chars().enumerate() {
            let shape = match c {
                'O' => Shape::new(Color::Orange),
                'P' => Shape::new(Color::Pink),
                'B' => Shape::new(Color::Blue),
                'G' => Shape::new(Color::Green),
                _ => return Err(format!("Invalid character at ({}, {})", i, j)),
            };

            row.push(shape);
        }

        grid.shapes.push(row);
    }

    Ok(grid)
}

pub fn parse_moves(input: &str) -> Result<Vec<(usize, usize)>, String> {
    let mut moves = vec![];

    for line in input.lines() {
        let parts = line.split(",").collect::<Vec<&str>>();

        if parts.len() != 2 {
            return Err("Invalid move format".to_string());
        }

        let row = parts[0].parse().map_err(|_| "Invalid row".to_string())?;
        let col = parts[1].parse().map_err(|_| "Invalid col".to_string())?;

        moves.push((row, col));
    }

    Ok(moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "OPG\nOPB\nOPG".to_string();
        let grid = parse_grid(&input).unwrap();

        assert_eq!(grid.shapes[0][0].color, Some(Color::Orange));
        assert_eq!(grid.shapes[0][1].color, Some(Color::Pink));
        assert_eq!(grid.shapes[0][2].color, Some(Color::Green));
        assert_eq!(grid.shapes[1][0].color, Some(Color::Orange));
        assert_eq!(grid.shapes[1][1].color, Some(Color::Pink));
        assert_eq!(grid.shapes[1][2].color, Some(Color::Blue));
        assert_eq!(grid.shapes[2][0].color, Some(Color::Orange));
        assert_eq!(grid.shapes[2][1].color, Some(Color::Pink));
        assert_eq!(grid.shapes[2][2].color, Some(Color::Green));
    }

    #[test]
    fn test_parse_invalid_character() {
        let input = "OPG\nOPX\nOPG".to_string();
        let result = parse_grid(&input);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid character at (1, 2)");
    }

    #[test]
    fn test_parse_moves() {
        let input = "0,0\n1,1\n2,2".to_string();
        let moves = parse_moves(&input).unwrap();

        assert_eq!(moves[0], (0, 0));
        assert_eq!(moves[1], (1, 1));
        assert_eq!(moves[2], (2, 2));
    }
}
