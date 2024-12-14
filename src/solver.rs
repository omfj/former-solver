use crate::grid::Grid;

pub struct Solver {
    pub grid: Grid,
}

impl Solver {
    /// Create a new solver with the given grid.
    pub fn new(grid: Grid) -> Solver {
        Solver { grid }
    }
}
