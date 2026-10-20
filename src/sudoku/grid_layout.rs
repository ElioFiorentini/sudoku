use crate::sudoku::grid::Grid;

#[derive(Debug)]
pub struct GridLayout {
    current_layout: Grid,
    layouts: Vec<(Grid, String)>,
}
