use std::{collections::HashSet, fmt};

use crate::shape::{Color, Shape};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
        self.remove_helper(row, col);
        self.tick();
    }

    fn remove_helper(&mut self, row: usize, col: usize) {
        let color = match self.shapes[row][col].color {
            Some(color) => color,
            None => return,
        };

        self.shapes[row][col].color = None;

        if row > 0 && self.shapes[row - 1][col].color == Some(color) {
            self.remove_helper(row - 1, col);
        }
        if row + 1 < self.shapes.len() && self.shapes[row + 1][col].color == Some(color) {
            self.remove_helper(row + 1, col);
        }
        if col > 0 && self.shapes[row][col - 1].color == Some(color) {
            self.remove_helper(row, col - 1);
        }
        if col + 1 < self.shapes[row].len() && self.shapes[row][col + 1].color == Some(color) {
            self.remove_helper(row, col + 1);
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

    /// Check if the grid is solved. Alias for `is_empty`.
    pub fn is_solved(&self) -> bool {
        self.is_empty()
    }

    /// Number of tiles that are empty.
    pub fn empty_tiles(&self) -> usize {
        self.shapes
            .iter()
            .map(|row| row.iter().filter(|shape| shape.color.is_none()).count())
            .sum()
    }

    /// A set of all valid moves on the grid.
    pub fn valid_moves(&self) -> HashSet<(usize, usize)> {
        let mut moves = HashSet::new();

        for row in 0..self.shapes.len() {
            for col in 0..self.shapes[0].len() {
                if self.shapes[row][col].color.is_some() {
                    moves.insert((row, col));
                }
            }
        }

        moves
    }

    /// Get the largest cluster of shapes.
    pub fn largest_cluster(&self) -> usize {
        let mut visited = vec![vec![false; self.shapes[0].len()]; self.shapes.len()];
        let mut max_cluster = 0;

        for row in 0..self.shapes.len() {
            for col in 0..self.shapes[0].len() {
                if visited[row][col] {
                    continue;
                }

                let color = match self.shapes[row][col].color {
                    Some(color) => color,
                    None => continue,
                };

                let mut cluster = 0;
                self.largest_cluster_helper(row, col, color, &mut visited, &mut cluster);

                max_cluster = max_cluster.max(cluster);
            }
        }

        max_cluster
    }

    fn largest_cluster_helper(
        &self,
        row: usize,
        col: usize,
        color: Color,
        visited: &mut Vec<Vec<bool>>,
        cluster: &mut usize,
    ) {
        if row >= self.shapes.len() || col >= self.shapes[0].len() {
            return;
        }

        if visited[row][col] {
            return;
        }

        if self.shapes[row][col].color != Some(color) {
            return;
        }

        visited[row][col] = true;
        *cluster += 1;

        if row > 0 {
            self.largest_cluster_helper(row - 1, col, color, visited, cluster);
        }
        if row + 1 < self.shapes.len() {
            self.largest_cluster_helper(row + 1, col, color, visited, cluster);
        }
        if col > 0 {
            self.largest_cluster_helper(row, col - 1, color, visited, cluster);
        }
        if col + 1 < self.shapes[0].len() {
            self.largest_cluster_helper(row, col + 1, color, visited, cluster);
        }
    }

    /// Get the number of clusters on the grid.
    pub fn cluster_count(&self) -> usize {
        let mut visited = vec![vec![false; self.shapes[0].len()]; self.shapes.len()];
        let mut count = 0;

        for row in 0..self.shapes.len() {
            for col in 0..self.shapes[0].len() {
                if visited[row][col] {
                    continue;
                }

                let color = match self.shapes[row][col].color {
                    Some(color) => color,
                    None => continue,
                };

                self.cluster_count_helper(row, col, color, &mut visited);
                count += 1;
            }
        }

        count
    }

    fn cluster_count_helper(
        &self,
        row: usize,
        col: usize,
        color: Color,
        visited: &mut Vec<Vec<bool>>,
    ) {
        if row >= self.shapes.len() || col >= self.shapes[0].len() {
            return;
        }

        if visited[row][col] {
            return;
        }

        if self.shapes[row][col].color != Some(color) {
            return;
        }

        visited[row][col] = true;

        if row > 0 {
            self.cluster_count_helper(row - 1, col, color, visited);
        }
        if row + 1 < self.shapes.len() {
            self.cluster_count_helper(row + 1, col, color, visited);
        }
        if col > 0 {
            self.cluster_count_helper(row, col - 1, color, visited);
        }
        if col + 1 < self.shapes[0].len() {
            self.cluster_count_helper(row, col + 1, color, visited);
        }
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
