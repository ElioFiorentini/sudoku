use super::grid::cell::Cell;
pub use super::grid::coordinate::Coordinate;
use std::fmt::Write;
mod cell;
mod coordinate;

/// Represent a 9x9 sudoku grid.
///
/// Each cell contain a digit between 0 and 9.\
/// 0 means the cell is empty.
/// # Examples
/// ```
/// # use sudoku_tools::Grid;
/// let grid = Grid::from([
///     [5, 3, 0, 0, 7, 0, 0, 0, 0],
///     [6, 0, 0, 1, 9, 5, 0, 0, 0],
///     [0, 9, 8, 0, 0, 0, 0, 6, 0],
///     [8, 0, 0, 0, 6, 0, 0, 0, 3],
///     [4, 0, 0, 8, 0, 3, 0, 0, 1],
///     [7, 0, 0, 0, 2, 0, 0, 0, 6],
///     [0, 6, 0, 0, 0, 0, 2, 8, 0],
///     [0, 0, 0, 4, 1, 9, 0, 0, 5],
///     [0, 0, 0, 0, 8, 0, 0, 7, 9],
/// ]);
/// print!("{grid}");
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct Grid {
    grid: [[Cell; 9]; 9],
}

impl std::ops::Index<Coordinate> for Grid {
    type Output = Cell;
    fn index(&self, index: Coordinate) -> &Self::Output {
        &self.grid[index.y as usize][index.x as usize]
    }
}
impl std::ops::IndexMut<Coordinate> for Grid {
    fn index_mut(&mut self, index: Coordinate) -> &mut Self::Output {
        &mut self.grid[index.y as usize][index.x as usize]
    }
}
impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn print_vline(s: &mut String, y: u8) -> std::fmt::Result {
            for x in 0..9 {
                if y % 3 == 0 {
                    write!(s, "+===")?;
                } else {
                    write!(s, "+---")?;
                }
            }
            write!(s, "+\n")
        }
        fn print_hline(s: &mut String, x: u8) -> std::fmt::Result {
            if x % 3 == 0 {
                write!(s, "‖")
            } else {
                write!(s, "|")
            }
        }
        let mut s = String::with_capacity(794);
        for row in Coordinate::rows() {
            print_vline(&mut s, row[0].y)?;
            for cell_coord in row {
                print_hline(&mut s, cell_coord.x)?;
                write!(s, " {} ", self[cell_coord])?;
            }
            write!(s, "‖\n")?;
        }
        print_vline(&mut s, 0)?;
        write!(f, "{s}")
    }
}
impl From<[[u8; 9]; 9]> for Grid {
    fn from(grid: [[u8; 9]; 9]) -> Self {
        let mut new_grid = Self::new();
        for y in 0..9 {
            for x in 0..9 {
                if grid[y][x] > 9 {
                    new_grid.grid[y][x].set(0);
                } else {
                    new_grid.grid[y][x].set(grid[y][x]);
                }
            }
        }
        new_grid
    }
}
impl Grid {
    pub fn new() -> Self {
        //! Create a new empty grid.
        //!
        //! The empty grid is filled with 0s.
        //!
        //! # Examples
        //! ```
        //! # use sudoku_tools::Grid;
        //! let grid = Grid::new();
        //! print!("{}", grid);
        //! ```
        Grid::default()
    }
    pub fn set(&mut self, coordinate: Coordinate, value: u8) -> Result<u8, &'static str> {
        //! Set the cell at `coordinate` with `value`.
        //!
        //! # Errors
        //! Returns an error if:
        //! - [`coordinate.check()`](Coordinate::check()) returns an error.
        //! - `value` exceeds 9
        //! - the cell at `coordinate` is locked
        //! # Examples
        //! ```
        //! # use sudoku_tools::{Coordinate, Grid};
        //! let mut grid = Grid::new();
        //! let coord = Coordinate { x: 4, y: 2 };
        //! let value = 1;
        //! match grid.set(coord, value) {
        //!     Ok(v) => println!("Successfully set the cell at {coord} with {value}, old value was {v}."),
        //!     Err(e) => println!("Error: {e}."),
        //! };
        //! ```
        coordinate.check()?;
        self[coordinate].set(value)
    }
    pub fn get(self, coordinate: Coordinate) -> Result<u8, &'static str> {
        //! Get the value in the cell at `coordinate`.
        //!
        //! # Errors
        //! Returns an error if [`coordinate.check()`](Coordinate::check()) returns an error.
        //! # Examples
        //! ```
        //! # use sudoku_tools::{Coordinate, Grid};
        //! let grid = Grid::new();
        //! match grid.get(Coordinate { x: 4, y: 2 }) {
        //!     Ok(v) => println!("The cell contains {v}."),
        //!     Err(e) => println!("Error: {e}."),
        //! };
        //! ```
        coordinate.check()?;
        Ok(self[coordinate].value())
    }
    pub fn lock(&mut self) {
        //! Lock every cell in the grid containing a value other than 0.
        //!
        //! Locked cells cannot be modified.
        //! # Examples
        //! ```
        //! # use sudoku_tools::{Coordinate, Grid};
        //! let mut grid = Grid::from([
        //!     [5, 3, 0, 0, 7, 0, 0, 0, 0],
        //!     [6, 0, 0, 1, 9, 5, 0, 0, 0],
        //!     [0, 9, 8, 0, 0, 0, 0, 6, 0],
        //!     [8, 0, 0, 0, 6, 0, 0, 0, 3],
        //!     [4, 0, 0, 8, 0, 3, 0, 0, 1],
        //!     [7, 0, 0, 0, 2, 0, 0, 0, 6],
        //!     [0, 6, 0, 0, 0, 0, 2, 8, 0],
        //!     [0, 0, 0, 4, 1, 9, 0, 0, 5],
        //!     [0, 0, 0, 0, 8, 0, 0, 7, 9],
        //! ]);
        //! grid.lock();
        //! match grid.set(Coordinate { x: 0, y: 0 }, 0) {
        //!     Ok(v) => println!("Successfully set the cell, old value was {v}."),
        //!     Err(e) => println!("Error: {e}."),
        //! };
        //! grid.unlock();
        //! match grid.set(Coordinate { x: 0, y: 0 }, 0) {
        //!     Ok(v) => println!("Successfully set the cell, old value was {v}."),
        //!     Err(e) => println!("Error: {e}."),
        //! };
        //! ```
        for row in &mut self.grid {
            for cell in row {
                if cell.is_filled() {
                    cell.lock();
                }
            }
        }
    }
    pub fn unlock(&mut self) {
        //! Unlock every cell in the grid.
        //!
        //! # Examples
        //! ```
        //! # use sudoku_tools::{Coordinate, Grid};
        //! let mut grid = Grid::from([
        //!     [5, 3, 0, 0, 7, 0, 0, 0, 0],
        //!     [6, 0, 0, 1, 9, 5, 0, 0, 0],
        //!     [0, 9, 8, 0, 0, 0, 0, 6, 0],
        //!     [8, 0, 0, 0, 6, 0, 0, 0, 3],
        //!     [4, 0, 0, 8, 0, 3, 0, 0, 1],
        //!     [7, 0, 0, 0, 2, 0, 0, 0, 6],
        //!     [0, 6, 0, 0, 0, 0, 2, 8, 0],
        //!     [0, 0, 0, 4, 1, 9, 0, 0, 5],
        //!     [0, 0, 0, 0, 8, 0, 0, 7, 9],
        //! ]);
        //! grid.lock();
        //! match grid.set(Coordinate { x: 0, y: 0 }, 0) {
        //!     Ok(v) => println!("Successfully set the cell, old value was {v}."),
        //!     Err(e) => println!("Error: {e}."),
        //! };
        //! grid.unlock();
        //! match grid.set(Coordinate { x: 0, y: 0 }, 0) {
        //!     Ok(v) => println!("Successfully set the cell, old value was {v}."),
        //!     Err(e) => println!("Error: {e}."),
        //! };
        //! ```
        for row in &mut self.grid {
            for cell in row {
                if cell.is_locked() {
                    cell.unlock();
                }
            }
        }
    }
    pub fn is_full(&self) -> bool {
        //! Check if the grid is full.
        //!
        //! The grid is full when no empty cell remains.
        //! # Examples
        //! ```
        //! # use sudoku_tools::{Coordinate, Grid};
        //! let mut grid = Grid::from([
        //!     [0, 1, 1, 1, 1, 1, 1, 1, 1],
        //!     [1, 1, 1, 1, 1, 1, 1, 1, 1],
        //!     [1, 1, 1, 1, 1, 1, 1, 1, 1],
        //!     [1, 1, 1, 1, 1, 1, 1, 1, 1],
        //!     [1, 1, 1, 1, 1, 1, 1, 1, 1],
        //!     [1, 1, 1, 1, 1, 1, 1, 1, 1],
        //!     [1, 1, 1, 1, 1, 1, 1, 1, 1],
        //!     [1, 1, 1, 1, 1, 1, 1, 1, 1],
        //!     [1, 1, 1, 1, 1, 1, 1, 1, 1],
        //! ]);
        //! println!("grid.is_full() = {}", grid.is_full());
        //! match grid.set(Coordinate { x: 0, y: 0 }, 1) {
        //!     Ok(v) => println!("Successfully set the cell, old value was {v}."),
        //!     Err(e) => println!("Error: {e}."),
        //! };
        //! println!("grid.is_full() = {}", grid.is_full());
        //! ```
        for row in self.grid {
            for cell in row {
                if cell.is_empty() {
                    return false;
                }
            }
        }
        true
    }
    pub fn display_errors(&self, errors: Vec<Coordinate>) {
        //! Display the grid and highlight cells containing errors.
        //!
        //! The highlighting uses ANSI Escape Codes to set the background of error containing cells to red.
        //! # Examples
        //! ```
        //! # use sudoku_tools::Grid;
        //! let incorrect_grid = Grid::from([
        //!     [5, 3, 4, 6, 7, 8, 9, 1, 2],
        //!     [6, 7, 6, 1, 9, 5, 3, 4, 8],
        //!     [1, 9, 8, 3, 4, 2, 5, 6, 7],
        //!     [8, 5, 9, 7, 6, 1, 4, 2, 3],
        //!     [4, 8, 6, 8, 5, 3, 7, 1, 1],
        //!     [7, 1, 3, 9, 2, 4, 8, 5, 6],
        //!     [9, 6, 1, 3, 3, 7, 2, 8, 4],
        //!     [2, 8, 7, 4, 1, 9, 6, 3, 5],
        //!     [3, 4, 5, 2, 8, 7, 1, 7, 9],
        //! ]);
        //! incorrect_grid.display_errors(incorrect_grid.errors());
        //! ```
        fn print_vline(s: &mut String, y: u8) {
            for x in 0..9 {
                if y % 3 == 0 {
                    write!(s, "+===");
                } else {
                    write!(s, "+---");
                }
            }
            write!(s, "+\n");
        }
        fn print_hline(s: &mut String, x: u8) {
            if x % 3 == 0 {
                write!(s, "‖");
            } else {
                write!(s, "|");
            }
        }
        let mut s = String::with_capacity(794 + errors.len() * 9);
        for row in Coordinate::rows() {
            print_vline(&mut s, row[0].y);
            for cell_coord in row {
                print_hline(&mut s, cell_coord.x);
                if errors.contains(&cell_coord) {
                    write!(s, "\x1b[41m {} \x1b[0m", self[cell_coord]);
                } else {
                    write!(s, " {} ", self[cell_coord]);
                }
            }
            write!(s, "‖\n");
        }
        print_vline(&mut s, 0);
        print!("{s}");
    }
    fn check_coordinates(&self, coordinates: [[Coordinate; 9]; 9], errors: &mut Vec<Coordinate>) {
        for grid_subsection in coordinates {
            let mut digits: [(u8, Coordinate); 10] = [(0, Coordinate::default()); 10];
            for position in grid_subsection {
                let value = self[position].value();
                if value == 0 {
                    continue;
                }
                if digits[value as usize].0 == value {
                    errors.push(digits[value as usize].1);
                    errors.push(position);
                } else {
                    digits[value as usize].0 = value;
                    digits[value as usize].1 = position;
                }
            }
        }
    }
    pub fn errors(&self) -> Vec<Coordinate> {
        //! Returns a vector of [Coordinate] containing the postitions of errors in the grid.
        //!
        //! # Examples
        //! ```
        //! # use sudoku_tools::Grid;
        //! let incorrect_grid = Grid::from([
        //!     [5, 3, 4, 6, 7, 8, 9, 1, 2],
        //!     [6, 7, 6, 1, 9, 5, 3, 4, 8],
        //!     [1, 9, 8, 3, 4, 2, 5, 6, 7],
        //!     [8, 5, 9, 7, 6, 1, 4, 2, 3],
        //!     [4, 8, 6, 8, 5, 3, 7, 1, 1],
        //!     [7, 1, 3, 9, 2, 4, 8, 5, 6],
        //!     [9, 6, 1, 3, 3, 7, 2, 8, 4],
        //!     [2, 8, 7, 4, 1, 9, 6, 3, 5],
        //!     [3, 4, 5, 2, 8, 7, 1, 7, 9],
        //! ]);
        //! incorrect_grid.display_errors(incorrect_grid.errors());
        //! dbg!(incorrect_grid.errors());
        //! ```
        let mut errors: Vec<Coordinate> = Vec::with_capacity(80);
        self.check_coordinates(Coordinate::boxes(), &mut errors);
        self.check_coordinates(Coordinate::columns(), &mut errors);
        self.check_coordinates(Coordinate::boxes(), &mut errors);
        errors.sort();
        errors.dedup();
        errors
    }
}
