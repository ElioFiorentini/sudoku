use crate::sudoku::cell::Cell;
pub use coordinate::Coordinate;
use std::fmt::Write;

pub mod coordinate;

#[derive(Clone, Copy, Debug)]
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
        self.display(Vec::with_capacity(0))
    }
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            grid: [[Cell::new(); 9]; 9],
        }
    }
    pub fn from(grid: [[u8; 9]; 9]) -> Self {
        let mut new_grid = Self::new();
        for y in 0..9 {
            for x in 0..9 {
                new_grid.grid[y][x].set(grid[y][x]);
            }
        }
        new_grid
    }
    pub fn set_cell(&mut self, coord: Coordinate, value: u8) -> Result<u8, &'static str> {
        self[coord].set(value)
    }
    pub fn get_cell(&mut self, coord: Coordinate) -> u8 {
        self[coord].value()
    }
    pub fn lock(&mut self) {
        for row in &mut self.grid {
            for cell in row {
                if cell.is_filled() {
                    cell.lock();
                }
            }
        }
    }
    pub fn unlock(&mut self) {
        for row in &mut self.grid {
            for cell in row {
                if cell.is_locked() {
                    cell.unlock();
                }
            }
        }
    }
    pub fn is_full(&self) -> bool {
        for row in self.grid {
            for cell in row {
                if cell.is_empty() {
                    return false;
                }
            }
        }
        true
    }

    pub fn display(&self, errors: Vec<Coordinate>) -> std::fmt::Result {
        fn print_vline(s: &mut String, y: usize) -> std::fmt::Result {
            for x in 0..9 {
                if y % 3 == 0 {
                    write!(s, "+===")?;
                } else {
                    write!(s, "+---")?;
                }
            }
            write!(s, "+\n")
        }
        let mut s = String::with_capacity(800);
        for row in Coordinate::rows() {
            print_vline(&mut s, row[0].y as usize)?;
            for cell_coord in row {
                if cell_coord.x % 3 == 0 {
                    write!(s, "‖");
                } else {
                    write!(s, "|");
                }
                if errors.contains(&cell_coord) {
                    write!(s, "\x1b[41m");
                }
                write!(s, " {} \x1b[0m", self[cell_coord])?;
            }
            write!(s, "‖\n")?;
        }
        print_vline(&mut s, 0)?;
        print!("{s}");
        Ok(())
    }

    fn check_coordinates(
        &self,
        coordinates: [[Coordinate; 9]; 9],
        errors: &mut Vec<Coordinate>,
    ) -> bool {
        let mut is_full = true;
        for grid_subsection in coordinates {
            let mut digits: [(u8, Coordinate); 10] = [(0, Coordinate::new()); 10];
            for position in grid_subsection {
                let value = self[position].value();
                if value == 0 {
                    is_full = false;
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
        is_full
    }
    pub fn check_errors(&self) -> Vec<Coordinate> {
        let mut errors: Vec<Coordinate> = Vec::with_capacity(20);
        let is_full = self.check_coordinates(Coordinate::boxes(), &mut errors)
            || self.check_coordinates(Coordinate::columns(), &mut errors)
            || self.check_coordinates(Coordinate::boxes(), &mut errors);
        errors.sort();
        errors.dedup();
        errors
    }
}
