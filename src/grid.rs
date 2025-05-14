use std::{collections::HashSet, fmt};

use crate::color::Color;

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Grid {
    pub colors: Vec<Vec<Option<Color>>>,
    pub moves: Vec<(usize, usize)>,
}

impl Grid {
    /// Create a new grid with the given colors.
    pub fn new(colors: Vec<Vec<Option<Color>>>) -> Grid {
        Grid {
            colors,
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
        if let Some(color) = self.colors[row][col] {
            self.colors[row][col] = None;

            for &(dx, dy) in &DIRS {
                let nx = (row as isize + dx) as usize;
                let ny = (col as isize + dy) as usize;

                if nx < self.colors.len() && ny < self.colors[0].len() {
                    if self.colors[nx][ny] == Some(color) {
                        self.remove_helper(nx, ny);
                    }
                }
            }
        }
    }

    /// Shift all the shapes that are above an empty space down.
    fn tick(&mut self) {
        for col in 0..self.colors[0].len() {
            let mut empty = 0;
            for row in (0..self.colors.len()).rev() {
                if self.colors[row][col].is_none() {
                    empty += 1;
                } else if empty > 0 {
                    self.colors[row + empty][col] = self.colors[row][col];
                    self.colors[row][col] = None;
                }
            }
        }
    }

    /// Check if the grid is empty/solved.
    pub fn is_empty(&self) -> bool {
        self.colors
            .iter()
            .all(|row| row.iter().all(|color| color.is_none()))
    }

    /// Check if the grid is solved. Alias for `is_empty`.
    pub fn is_solved(&self) -> bool {
        self.is_empty()
    }

    /// Number of tiles that are empty.
    pub fn empty_tiles(&self) -> usize {
        self.colors
            .iter()
            .map(|row| row.iter().filter(|color| color.is_none()).count())
            .sum()
    }

    /// A set of all valid moves on the grid.
    pub fn valid_moves(&self) -> HashSet<(usize, usize)> {
        let mut moves = HashSet::new();

        for row in 0..self.colors.len() {
            for col in 0..self.colors[0].len() {
                if self.colors[row][col].is_some() {
                    moves.insert((row, col));
                }
            }
        }

        moves
    }

    /// Get the largest cluster of shapes.
    pub fn largest_cluster(&self) -> usize {
        let mut visited = vec![vec![false; self.colors[0].len()]; self.colors.len()];
        let mut max_cluster = 0;

        for row in 0..self.colors.len() {
            for col in 0..self.colors[0].len() {
                if visited[row][col] {
                    continue;
                }

                let color = match self.colors[row][col] {
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
        if row >= self.colors.len() || col >= self.colors[0].len() {
            return;
        }

        if visited[row][col] {
            return;
        }

        if self.colors[row][col] != Some(color) {
            return;
        }

        visited[row][col] = true;
        *cluster += 1;

        if row > 0 {
            self.largest_cluster_helper(row - 1, col, color, visited, cluster);
        }
        if row + 1 < self.colors.len() {
            self.largest_cluster_helper(row + 1, col, color, visited, cluster);
        }
        if col > 0 {
            self.largest_cluster_helper(row, col - 1, color, visited, cluster);
        }
        if col + 1 < self.colors[0].len() {
            self.largest_cluster_helper(row, col + 1, color, visited, cluster);
        }
    }

    /// Get the number of clusters on the grid.
    pub fn cluster_count(&self) -> usize {
        let mut visited = vec![vec![false; self.colors[0].len()]; self.colors.len()];
        let mut count = 0;

        for row in 0..self.colors.len() {
            for col in 0..self.colors[0].len() {
                if visited[row][col] {
                    continue;
                }

                let color = match self.colors[row][col] {
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
        if row >= self.colors.len() || col >= self.colors[0].len() {
            return;
        }

        if visited[row][col] {
            return;
        }

        if self.colors[row][col] != Some(color) {
            return;
        }

        visited[row][col] = true;

        if row > 0 {
            self.cluster_count_helper(row - 1, col, color, visited);
        }
        if row + 1 < self.colors.len() {
            self.cluster_count_helper(row + 1, col, color, visited);
        }
        if col > 0 {
            self.cluster_count_helper(row, col - 1, color, visited);
        }
        if col + 1 < self.colors[0].len() {
            self.cluster_count_helper(row, col + 1, color, visited);
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        result.push_str(&format!("+{}+\n", "-".repeat(self.colors[0].len())));
        for row in &self.colors {
            result.push('|');
            for shape in row {
                let color = match shape {
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
        result.push_str(&format!("+{}+\n", "-".repeat(self.colors[0].len())));

        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove() {
        let shapes = vec![vec![
            Some(Color::Orange),
            Some(Color::Orange),
            Some(Color::Orange),
        ]];

        let mut grid = Grid::new(shapes);
        grid.remove(0, 0);

        let expected = vec![vec![None, None, None]];

        assert_eq!(grid.colors, expected);
    }

    #[test]
    fn test_gravity() {
        let shapes = vec![
            vec![Some(Color::Orange)],
            vec![Some(Color::Orange)],
            vec![Some(Color::Blue)],
        ];

        let mut grid = Grid::new(shapes);
        grid.remove(2, 0);

        let expected = vec![
            vec![None],
            vec![Some(Color::Orange)],
            vec![Some(Color::Orange)],
        ];

        assert_eq!(grid.colors, expected);
    }
}
