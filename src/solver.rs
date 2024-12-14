use crate::grid::Grid;
use std::collections::HashSet;

type Row = usize;
type Col = usize;
type Score = usize;

pub struct Solver {
    pub grid: Grid,
    pub beam_max_depth: usize,
    pub beam_width: usize,
}

impl Solver {
    /// Create a new solver with the given grid.
    pub fn new(grid: Grid) -> Solver {
        Solver {
            grid,
            beam_max_depth: 17,
            beam_width: 4,
        }
    }

    /// Solve the grid using beam search.
    pub fn beam_search(&self) -> Option<Vec<(Row, Col)>> {
        let moves = 0;
        let grid = self.grid.clone();
        let mut candidates: Vec<(Grid, Vec<(Row, Col)>)> = vec![(grid, Vec::new())];
        let mut seen_grids: HashSet<Grid> = HashSet::new();

        for depth in 0..self.beam_max_depth {
            let mut next_candidates: Vec<(Grid, Vec<(Row, Col)>, Score)> = Vec::new();

            for (grid, path) in candidates.iter() {
                for (row, col) in grid.valid_moves() {
                    let mut new_grid = grid.clone();
                    new_grid.remove(row, col);

                    if seen_grids.contains(&new_grid) {
                        continue;
                    }

                    seen_grids.insert(new_grid.clone());

                    let mut new_path = path.clone();
                    new_path.push((row, col));

                    if new_grid.is_solved() {
                        return Some(new_path);
                    }

                    let score = lookahead_score(&new_grid, 3, moves + 1);
                    next_candidates.push((new_grid, new_path, score));
                }
            }

            next_candidates.sort_by_key(|&(_, _, score)| -(score as isize));

            next_candidates.truncate(self.beam_width);

            candidates = next_candidates
                .into_iter()
                .map(|(grid, path, _)| (grid, path))
                .collect();

            if candidates.is_empty() {
                println!("No candidates left, breaking early at depth {}.", depth);
                break;
            }
        }

        for (grid, path) in candidates {
            if grid.is_solved() {
                return Some(path);
            }
        }

        None
    }
}

fn score(grid: &Grid, moves: usize) -> Score {
    let empty_count = grid.empty_tiles();
    let largest_cluster_size = grid.largest_cluster();

    (empty_count + largest_cluster_size) * 10 / (moves + 1)
}

fn lookahead_score(grid: &Grid, depth: usize, current_moves: usize) -> Score {
    if depth == 0 {
        return score(grid, current_moves);
    }

    let mut best_score = 0;
    for (row, col) in grid.valid_moves() {
        let mut new_grid = grid.clone();
        new_grid.remove(row, col);
        let potential_score = lookahead_score(&new_grid, depth - 1, current_moves + 1);
        if potential_score > best_score {
            best_score = potential_score;
        }
    }

    best_score
}
