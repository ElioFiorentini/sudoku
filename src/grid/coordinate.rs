/// Represents 2d coordinates in a sudoku grid.
///
/// Contains an `x` value and a `y` value.\
/// These values are represented using [`u8`] as they are supposed
/// to be used with a sudoku [`Grid`](super::Grid)
/// which can only take coordinates between 0 and 8.
///
/// Coordinates can be used to [`Index`](std::ops::Index) a [`Grid`](super::Grid).\
/// Beware when using this as [`Index`](std::ops::Index) may panic if the coordinates are out of bounds of the [`Grid`](super::Grid).
/// # Examples
/// ```
/// # use sudoku_tools::{Coordinate, Grid};
/// let grid = Grid::new();
/// let coord = Coordinate { x: 4, y: 2 };
/// println!("The value at {} is {}.", coord, grid[coord].value());
/// ```
/// or
/// ```
/// # use sudoku_tools::{Coordinate, Grid};
/// let grid = Grid::new();
/// let coord = Coordinate { x: 4, y: 2 };
/// match grid.get(coord) {
///     Ok(value) => println!("The value at {} is {}.", coord, value),
///     Err(e) => eprintln!("Error: {e}."),
/// }
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct Coordinate {
    /// The x coordinate in a sudoku [`Grid`](super::Grid).
    pub x: u8,
    /// The y coordinate in a sudoku [`Grid`](super::Grid).
    pub y: u8,
}
impl std::convert::From<(u8, u8)> for Coordinate {
    fn from(value: (u8, u8)) -> Self {
        Coordinate {
            x: value.0,
            y: value.1,
        }
    }
}
impl std::fmt::Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Coordinate {
    pub fn new(x: u8, y: u8) -> Self {
        //! Create new coordinates (x, y).
        //!
        //! Doing `Coordinate::new(4, 2)` is the same as doing `Coordinate { x: 4, y: 2 }`.
        //! # Examples
        //! ```
        //! # use sudoku_tools::Coordinate;
        //! let coord = Coordinate::new(4, 2);
        //! print!("{}", coord);
        //! ```
        Coordinate { x: x, y: y }
    }

    pub fn check(self) -> Result<(), &'static str> {
        //! Checks if the coordinates are >= 0 and <= 8.
        //!
        //! # Errors
        //! Returns an error if `x` or `y` is > 8.
        //! # Examples
        //! ```
        //! # use sudoku_tools::Coordinate;
        //! let coord = Coordinate::new(4, 2);
        //! match coord.check() {
        //!     Err(e) => eprintln!("Error: {e}"),
        //!     _ => (),
        //! }
        //! let coord = Coordinate::new(23, 16);
        //! match coord.check() {
        //!     Err(e) => eprintln!("Error: {e}"),
        //!     _ => (),
        //! }
        //! ```
        if self.x > 8 || self.y > 8 {
            Err("out of bound coordinates, must be >= 0 and <= 8")
        } else {
            Ok(())
        }
    }

    pub fn rows() -> [[Coordinate; 9]; 9] {
        //! Return every [`Grid`](super::Grid) coordinate bundled in arrays of rows.
        //!
        //! # Examples
        //! ```
        //! # use sudoku_tools::Coordinate;
        //! for row in Coordinate::rows() {
        //!     for cell_coord in row {
        //!         print!("{} ", cell_coord);
        //!     }
        //!     println!();
        //! }
        //! ```
        /*
            let mut rows: [[Coordinate; 9]; 9] = [[Coordinate::new(); 9]; 9];
            for y in 0..9u8 {
                for x in 0..9u8 {
                    rows[y as usize][x as usize].x = x;
                    rows[y as usize][x as usize].y = y;
                }
            }
            rows
        */
        // Pre computed with the above code.
        [
            [
                Coordinate { x: 0, y: 0 },
                Coordinate { x: 1, y: 0 },
                Coordinate { x: 2, y: 0 },
                Coordinate { x: 3, y: 0 },
                Coordinate { x: 4, y: 0 },
                Coordinate { x: 5, y: 0 },
                Coordinate { x: 6, y: 0 },
                Coordinate { x: 7, y: 0 },
                Coordinate { x: 8, y: 0 },
            ],
            [
                Coordinate { x: 0, y: 1 },
                Coordinate { x: 1, y: 1 },
                Coordinate { x: 2, y: 1 },
                Coordinate { x: 3, y: 1 },
                Coordinate { x: 4, y: 1 },
                Coordinate { x: 5, y: 1 },
                Coordinate { x: 6, y: 1 },
                Coordinate { x: 7, y: 1 },
                Coordinate { x: 8, y: 1 },
            ],
            [
                Coordinate { x: 0, y: 2 },
                Coordinate { x: 1, y: 2 },
                Coordinate { x: 2, y: 2 },
                Coordinate { x: 3, y: 2 },
                Coordinate { x: 4, y: 2 },
                Coordinate { x: 5, y: 2 },
                Coordinate { x: 6, y: 2 },
                Coordinate { x: 7, y: 2 },
                Coordinate { x: 8, y: 2 },
            ],
            [
                Coordinate { x: 0, y: 3 },
                Coordinate { x: 1, y: 3 },
                Coordinate { x: 2, y: 3 },
                Coordinate { x: 3, y: 3 },
                Coordinate { x: 4, y: 3 },
                Coordinate { x: 5, y: 3 },
                Coordinate { x: 6, y: 3 },
                Coordinate { x: 7, y: 3 },
                Coordinate { x: 8, y: 3 },
            ],
            [
                Coordinate { x: 0, y: 4 },
                Coordinate { x: 1, y: 4 },
                Coordinate { x: 2, y: 4 },
                Coordinate { x: 3, y: 4 },
                Coordinate { x: 4, y: 4 },
                Coordinate { x: 5, y: 4 },
                Coordinate { x: 6, y: 4 },
                Coordinate { x: 7, y: 4 },
                Coordinate { x: 8, y: 4 },
            ],
            [
                Coordinate { x: 0, y: 5 },
                Coordinate { x: 1, y: 5 },
                Coordinate { x: 2, y: 5 },
                Coordinate { x: 3, y: 5 },
                Coordinate { x: 4, y: 5 },
                Coordinate { x: 5, y: 5 },
                Coordinate { x: 6, y: 5 },
                Coordinate { x: 7, y: 5 },
                Coordinate { x: 8, y: 5 },
            ],
            [
                Coordinate { x: 0, y: 6 },
                Coordinate { x: 1, y: 6 },
                Coordinate { x: 2, y: 6 },
                Coordinate { x: 3, y: 6 },
                Coordinate { x: 4, y: 6 },
                Coordinate { x: 5, y: 6 },
                Coordinate { x: 6, y: 6 },
                Coordinate { x: 7, y: 6 },
                Coordinate { x: 8, y: 6 },
            ],
            [
                Coordinate { x: 0, y: 7 },
                Coordinate { x: 1, y: 7 },
                Coordinate { x: 2, y: 7 },
                Coordinate { x: 3, y: 7 },
                Coordinate { x: 4, y: 7 },
                Coordinate { x: 5, y: 7 },
                Coordinate { x: 6, y: 7 },
                Coordinate { x: 7, y: 7 },
                Coordinate { x: 8, y: 7 },
            ],
            [
                Coordinate { x: 0, y: 8 },
                Coordinate { x: 1, y: 8 },
                Coordinate { x: 2, y: 8 },
                Coordinate { x: 3, y: 8 },
                Coordinate { x: 4, y: 8 },
                Coordinate { x: 5, y: 8 },
                Coordinate { x: 6, y: 8 },
                Coordinate { x: 7, y: 8 },
                Coordinate { x: 8, y: 8 },
            ],
        ]
    }

    pub fn columns() -> [[Coordinate; 9]; 9] {
        //! Return every [`Grid`](super::Grid) coordinate bundled in arrays of columns.
        //!
        //! # Examples
        //! ```
        //! # use sudoku_tools::Coordinate;
        //! for col in Coordinate::columns() {
        //!     for cell_coord in col {
        //!         print!("{} ", cell_coord);
        //!     }
        //!     println!();
        //! }
        //! ```
        /*
            let mut cols: [[Coordinate; 9]; 9] = [[Coordinate::new(); 9]; 9];
            for y in 0..9u8 {
                for x in 0..9u8 {
                    cols[x as usize][y as usize].x = x;
                    cols[x as usize][y as usize].y = y;
                }
            }
            cols
        */
        // Pre computed with the above code.
        [
            [
                Coordinate { x: 0, y: 0 },
                Coordinate { x: 0, y: 1 },
                Coordinate { x: 0, y: 2 },
                Coordinate { x: 0, y: 3 },
                Coordinate { x: 0, y: 4 },
                Coordinate { x: 0, y: 5 },
                Coordinate { x: 0, y: 6 },
                Coordinate { x: 0, y: 7 },
                Coordinate { x: 0, y: 8 },
            ],
            [
                Coordinate { x: 1, y: 0 },
                Coordinate { x: 1, y: 1 },
                Coordinate { x: 1, y: 2 },
                Coordinate { x: 1, y: 3 },
                Coordinate { x: 1, y: 4 },
                Coordinate { x: 1, y: 5 },
                Coordinate { x: 1, y: 6 },
                Coordinate { x: 1, y: 7 },
                Coordinate { x: 1, y: 8 },
            ],
            [
                Coordinate { x: 2, y: 0 },
                Coordinate { x: 2, y: 1 },
                Coordinate { x: 2, y: 2 },
                Coordinate { x: 2, y: 3 },
                Coordinate { x: 2, y: 4 },
                Coordinate { x: 2, y: 5 },
                Coordinate { x: 2, y: 6 },
                Coordinate { x: 2, y: 7 },
                Coordinate { x: 2, y: 8 },
            ],
            [
                Coordinate { x: 3, y: 0 },
                Coordinate { x: 3, y: 1 },
                Coordinate { x: 3, y: 2 },
                Coordinate { x: 3, y: 3 },
                Coordinate { x: 3, y: 4 },
                Coordinate { x: 3, y: 5 },
                Coordinate { x: 3, y: 6 },
                Coordinate { x: 3, y: 7 },
                Coordinate { x: 3, y: 8 },
            ],
            [
                Coordinate { x: 4, y: 0 },
                Coordinate { x: 4, y: 1 },
                Coordinate { x: 4, y: 2 },
                Coordinate { x: 4, y: 3 },
                Coordinate { x: 4, y: 4 },
                Coordinate { x: 4, y: 5 },
                Coordinate { x: 4, y: 6 },
                Coordinate { x: 4, y: 7 },
                Coordinate { x: 4, y: 8 },
            ],
            [
                Coordinate { x: 5, y: 0 },
                Coordinate { x: 5, y: 1 },
                Coordinate { x: 5, y: 2 },
                Coordinate { x: 5, y: 3 },
                Coordinate { x: 5, y: 4 },
                Coordinate { x: 5, y: 5 },
                Coordinate { x: 5, y: 6 },
                Coordinate { x: 5, y: 7 },
                Coordinate { x: 5, y: 8 },
            ],
            [
                Coordinate { x: 6, y: 0 },
                Coordinate { x: 6, y: 1 },
                Coordinate { x: 6, y: 2 },
                Coordinate { x: 6, y: 3 },
                Coordinate { x: 6, y: 4 },
                Coordinate { x: 6, y: 5 },
                Coordinate { x: 6, y: 6 },
                Coordinate { x: 6, y: 7 },
                Coordinate { x: 6, y: 8 },
            ],
            [
                Coordinate { x: 7, y: 0 },
                Coordinate { x: 7, y: 1 },
                Coordinate { x: 7, y: 2 },
                Coordinate { x: 7, y: 3 },
                Coordinate { x: 7, y: 4 },
                Coordinate { x: 7, y: 5 },
                Coordinate { x: 7, y: 6 },
                Coordinate { x: 7, y: 7 },
                Coordinate { x: 7, y: 8 },
            ],
            [
                Coordinate { x: 8, y: 0 },
                Coordinate { x: 8, y: 1 },
                Coordinate { x: 8, y: 2 },
                Coordinate { x: 8, y: 3 },
                Coordinate { x: 8, y: 4 },
                Coordinate { x: 8, y: 5 },
                Coordinate { x: 8, y: 6 },
                Coordinate { x: 8, y: 7 },
                Coordinate { x: 8, y: 8 },
            ],
        ]
    }

    pub fn boxes() -> [[Coordinate; 9]; 9] {
        //! Return every [`Grid`](super::Grid) coordinate bundled in arrays of sudoku boxes.
        //!
        //! # Examples
        //! ```
        //! # use sudoku_tools::Coordinate;
        //! for sudoku_box in Coordinate::boxes() {
        //!     for cell_coord in sudoku_box {
        //!         print!("{} ", cell_coord);
        //!     }
        //!     println!();
        //! }
        //! ```
        /*
            let mut boxes: [[Coordinate; 9]; 9] = [[Coordinate::new(); 9]; 9];
            let mut box_j: usize = 0;
            for y in (0..9usize).step_by(3) {
                for x in (0..9usize).step_by(3) {
                    let mut box_i: usize = 0;
                    for j in 0..3usize {
                        for i in 0..3usize {
                            let value = boxes[y + j][x + i];
                            boxes[box_j][box_i].x = (x + i) as u8;
                            boxes[box_j][box_i].y = (y + j) as u8;
                            box_i += 1;
                        }
                    }
                    box_j += 1;
                }
            }
            boxes
        */
        // Pre computed with the above code.
        [
            [
                Coordinate { x: 0, y: 0 },
                Coordinate { x: 1, y: 0 },
                Coordinate { x: 2, y: 0 },
                Coordinate { x: 0, y: 1 },
                Coordinate { x: 1, y: 1 },
                Coordinate { x: 2, y: 1 },
                Coordinate { x: 0, y: 2 },
                Coordinate { x: 1, y: 2 },
                Coordinate { x: 2, y: 2 },
            ],
            [
                Coordinate { x: 3, y: 0 },
                Coordinate { x: 4, y: 0 },
                Coordinate { x: 5, y: 0 },
                Coordinate { x: 3, y: 1 },
                Coordinate { x: 4, y: 1 },
                Coordinate { x: 5, y: 1 },
                Coordinate { x: 3, y: 2 },
                Coordinate { x: 4, y: 2 },
                Coordinate { x: 5, y: 2 },
            ],
            [
                Coordinate { x: 6, y: 0 },
                Coordinate { x: 7, y: 0 },
                Coordinate { x: 8, y: 0 },
                Coordinate { x: 6, y: 1 },
                Coordinate { x: 7, y: 1 },
                Coordinate { x: 8, y: 1 },
                Coordinate { x: 6, y: 2 },
                Coordinate { x: 7, y: 2 },
                Coordinate { x: 8, y: 2 },
            ],
            [
                Coordinate { x: 0, y: 3 },
                Coordinate { x: 1, y: 3 },
                Coordinate { x: 2, y: 3 },
                Coordinate { x: 0, y: 4 },
                Coordinate { x: 1, y: 4 },
                Coordinate { x: 2, y: 4 },
                Coordinate { x: 0, y: 5 },
                Coordinate { x: 1, y: 5 },
                Coordinate { x: 2, y: 5 },
            ],
            [
                Coordinate { x: 3, y: 3 },
                Coordinate { x: 4, y: 3 },
                Coordinate { x: 5, y: 3 },
                Coordinate { x: 3, y: 4 },
                Coordinate { x: 4, y: 4 },
                Coordinate { x: 5, y: 4 },
                Coordinate { x: 3, y: 5 },
                Coordinate { x: 4, y: 5 },
                Coordinate { x: 5, y: 5 },
            ],
            [
                Coordinate { x: 6, y: 3 },
                Coordinate { x: 7, y: 3 },
                Coordinate { x: 8, y: 3 },
                Coordinate { x: 6, y: 4 },
                Coordinate { x: 7, y: 4 },
                Coordinate { x: 8, y: 4 },
                Coordinate { x: 6, y: 5 },
                Coordinate { x: 7, y: 5 },
                Coordinate { x: 8, y: 5 },
            ],
            [
                Coordinate { x: 0, y: 6 },
                Coordinate { x: 1, y: 6 },
                Coordinate { x: 2, y: 6 },
                Coordinate { x: 0, y: 7 },
                Coordinate { x: 1, y: 7 },
                Coordinate { x: 2, y: 7 },
                Coordinate { x: 0, y: 8 },
                Coordinate { x: 1, y: 8 },
                Coordinate { x: 2, y: 8 },
            ],
            [
                Coordinate { x: 3, y: 6 },
                Coordinate { x: 4, y: 6 },
                Coordinate { x: 5, y: 6 },
                Coordinate { x: 3, y: 7 },
                Coordinate { x: 4, y: 7 },
                Coordinate { x: 5, y: 7 },
                Coordinate { x: 3, y: 8 },
                Coordinate { x: 4, y: 8 },
                Coordinate { x: 5, y: 8 },
            ],
            [
                Coordinate { x: 6, y: 6 },
                Coordinate { x: 7, y: 6 },
                Coordinate { x: 8, y: 6 },
                Coordinate { x: 6, y: 7 },
                Coordinate { x: 7, y: 7 },
                Coordinate { x: 8, y: 7 },
                Coordinate { x: 6, y: 8 },
                Coordinate { x: 7, y: 8 },
                Coordinate { x: 8, y: 8 },
            ],
        ]
    }
}
