use std::fmt;

use crate::shape::{Color, Shape};

#[derive(Clone, Debug, PartialEq)]
pub struct Grid {
    pub shapes: Vec<Vec<Shape>>,
    pub moves: Vec<(usize, usize)>,
}

impl Grid {
    /// Create a new grid with the given colors.
    pub fn new(shapes: Vec<Vec<Shape>>) -> Grid {
        Grid {
            shapes,
            moves: Vec::new(),
        }
    }

    /// Remove the shape at the given row and column,
    /// and all adjacent shapes of the same color.
    pub fn remove(&mut self, row: usize, col: usize) {
        self.moves.push((row, col));
        self._remove(row, col);
        self.tick();
    }

    /// Internal remove
    fn _remove(&mut self, row: usize, col: usize) {
        let color = match self.shapes[row][col].color {
            Some(color) => color,
            None => return,
        };

        self.shapes[row][col].color = None;

        if row > 0 && self.shapes[row - 1][col].color == Some(color) {
            self._remove(row - 1, col);
        }
        if row + 1 < self.shapes.len() && self.shapes[row + 1][col].color == Some(color) {
            self._remove(row + 1, col);
        }
        if col > 0 && self.shapes[row][col - 1].color == Some(color) {
            self._remove(row, col - 1);
        }
        if col + 1 < self.shapes[row].len() && self.shapes[row][col + 1].color == Some(color) {
            self._remove(row, col + 1);
        }
    }

    /// Shift all the shapes that are above an empty space down.
    fn tick(&mut self) {
        for col in 0..self.shapes[0].len() {
            let mut empty = 0;
            for row in (0..self.shapes.len()).rev() {
                if self.shapes[row][col].color.is_none() {
                    empty += 1;
                } else if empty > 0 {
                    self.shapes[row + empty][col] = self.shapes[row][col];
                    self.shapes[row][col].color = None;
                }
            }
        }
    }

    /// Check if the grid is empty/solved.
    pub fn is_empty(&self) -> bool {
        self.shapes
            .iter()
            .all(|row| row.iter().all(|shape| shape.color.is_none()))
    }

    /// Return the number of tiles that are empty.
    pub fn empty_tiles(&self) -> usize {
        self.shapes
            .iter()
            .map(|row| row.iter().filter(|shape| shape.color.is_none()).count())
            .sum()
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        result.push_str(&format!("+{}+\n", "-".repeat(self.shapes[0].len())));
        for row in &self.shapes {
            result.push('|');
            for shape in row {
                let color = match shape.color {
                    Some(Color::Orange) => "O",
                    Some(Color::Pink) => "P",
                    Some(Color::Blue) => "B",
                    Some(Color::Green) => "G",
                    None => " ",
                };
                result.push_str(color);
            }
            result.push_str("|\n");
        }
        result.push_str(&format!("+{}+\n", "-".repeat(self.shapes[0].len())));

        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove() {
        let shapes = vec![vec![
            Shape::new(Color::Orange),
            Shape::new(Color::Orange),
            Shape::new(Color::Orange),
        ]];

        let mut grid = Grid::new(shapes);
        grid.remove(0, 0);

        let expected = vec![vec![
            Shape { color: None },
            Shape { color: None },
            Shape { color: None },
        ]];

        assert_eq!(grid.shapes, expected);
    }

    #[test]
    fn test_gravity() {
        let shapes = vec![
            vec![Shape::new(Color::Orange)],
            vec![Shape::new(Color::Orange)],
            vec![Shape::new(Color::Blue)],
        ];

        let mut grid = Grid::new(shapes);
        grid.remove(2, 0);

        let expected = vec![
            vec![Shape { color: None }],
            vec![Shape::new(Color::Orange)],
            vec![Shape::new(Color::Orange)],
        ];

        assert_eq!(grid.shapes, expected);
    }
}
