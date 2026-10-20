#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coordinate {
    pub x: u8,
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
    pub fn new() -> Self {
        Coordinate { x: 0, y: 0 }
    }
    pub fn rows() -> [[Coordinate; 9]; 9] {
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
