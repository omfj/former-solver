use crate::grid::Grid;
use crate::shape::{Color, Shape};

pub struct Parser {
    pub input: String,
}

impl Parser {
    /// Create a new parser with the given input.
    pub fn new(input: String) -> Parser {
        Parser { input }
    }

    /// Parse the input and return a grid.
    pub fn parse(&self) -> Result<Grid, String> {
        let mut grid = Grid::new(vec![]);

        for (i, line) in self.input.lines().enumerate() {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "OPG\nOPB\nOPG".to_string();
        let parser = Parser::new(input);
        let grid = parser.parse().unwrap();

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
        let parser = Parser::new(input);
        let result = parser.parse();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid character at (1, 2)");
    }
}
