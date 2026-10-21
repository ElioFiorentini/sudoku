//! Sudoku solving tools.
//!
//! This crate provides tools to help you solve sudoku puzzles, not solve it directly.
//! Provided tools are:
//! - an error checker
//! - a way to lock the grid
//! - a way to save and load grid states

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(rustdoc::unescaped_backticks)]
#[allow(dead_code, unused)]
/// Sudoku solving tools.
pub mod sudoku {
    /// Sudoku grid representation.
    pub mod grid;
    // Sudoku grid states handler.
    mod grid_layout;
}
pub use sudoku::grid::Coordinate;
pub use sudoku::grid::Grid;
