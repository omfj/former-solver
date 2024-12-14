use crate::grid::Grid;

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
            beam_max_depth: 100,
            beam_width: 100,
        }
    }

    /// Solve the grid using beam search.
    pub fn beam_search(&self) -> Vec<(Row, Col)> {
        self.beam_search_helper(self.grid.clone(), 0)
    }

    fn beam_search_helper(&self, grid: Grid, moves: usize) -> Vec<(Row, Col)> {
        let mut candidates: Vec<(Grid, Vec<(Row, Col)>)> = vec![(grid, Vec::new())];

        for depth in 0..self.beam_max_depth {
            let mut next_candidates: Vec<(Grid, Vec<(Row, Col)>, Score)> = Vec::new();

            for (grid, path) in candidates.iter() {
                for (row, col) in grid.valid_moves() {
                    let mut new_grid = grid.clone();
                    new_grid.remove(row, col);

                    let mut new_path = path.clone();
                    new_path.push((row, col));

                    if new_grid.is_solved() {
                        return new_path;
                    }

                    let score = score(&new_grid, moves + 1);
                    next_candidates.push((new_grid, new_path, score));
                }
            }

            // Sort by score (highest scores first)
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

        if let Some((_, best_path)) = candidates.first() {
            best_path.clone()
        } else {
            Vec::new()
        }
    }
}

fn score(grid: &Grid, moves: usize) -> Score {
    if moves == 0 {
        return 0;
    }
    grid.empty_tiles() / moves
}
