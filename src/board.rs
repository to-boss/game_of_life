use crate::{BOARD_SIZE, CELL_SIZE};
use macroquad::prelude::*;

pub struct Board {
    cells: [[Option<bool>; BOARD_SIZE]; BOARD_SIZE],
    offset: usize,
}

impl Board {
    pub fn init() -> Self {
        Self {
            cells: [[None; BOARD_SIZE]; BOARD_SIZE],
            offset: 40,
        }
    }

    pub fn reset(&mut self) {
        self.cells = [[None; BOARD_SIZE]; BOARD_SIZE];
    }

    pub fn fill_random(&mut self) {
        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                let num = rand::gen_range(0, 2);
                self.cells[x][y] = if num == 0 { Some(true) } else { None };
            }
        }
    }

    pub fn tick(&mut self) {
        let mut new_cells = [[None; BOARD_SIZE]; BOARD_SIZE];

        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                let mut neighbors = 0;

                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }

                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;

                        if nx < 0 || nx >= BOARD_SIZE as i32 {
                            continue;
                        }

                        if ny < 0 || ny >= BOARD_SIZE as i32 {
                            continue;
                        }

                        if self.cells[nx as usize][ny as usize].is_some() {
                            neighbors += 1;
                        }
                    }
                }

                new_cells[x][y] = match (self.cells[x][y], neighbors) {
                    (Some(true), 2) => Some(true),
                    (_, 3) => Some(true),
                    _ => None,
                };
            }
        }

        self.cells = new_cells;
    }

    pub fn switch_cell(&mut self, x: usize, y: usize) {
        self.cells[x][y] = match self.cells[x][y] {
            Some(_) => None,
            None => Some(true),
        };
    }

    /// Returns the mouse to cell position of this [`Board`].
    /// Returns none if not in the board.
    pub fn mouse_to_cell_position(&self) -> Option<(usize, usize)> {
        let size = CELL_SIZE as f32;
        let board_size = BOARD_SIZE as f32;
        let offset = self.offset as f32;

        let (x, y) = mouse_position();
        let x = (x - offset) / size;
        let y = (y - offset) / size;

        let out_of_lower_bounds = x < 0. || y < 0.;
        let out_of_upper_bounds = x >= board_size || y >= board_size;

        if out_of_lower_bounds || out_of_upper_bounds {
            return None;
        }

        Some((x as usize, y as usize))
    }

    pub fn draw(&self) {
        self.draw_grid();
        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                if self.cells[x][y].is_some() {
                    self.draw_cell(x, y, self.offset);
                }
            }
        }
    }

    fn draw_grid(&self) {
        let max = ((BOARD_SIZE * CELL_SIZE) + self.offset) as f32;
        let offset = self.offset as f32;

        for i in 0..(BOARD_SIZE + 1) as usize {
            let i = ((i * CELL_SIZE) + self.offset) as f32;
            draw_line(i, offset, i, max, 1., GRAY);
            draw_line(offset, i, max, i, 1., GRAY);
        }
    }

    fn draw_cell(&self, x: usize, y: usize, offset: usize) {
        let x = ((x * CELL_SIZE) + offset) as f32;
        let y = ((y * CELL_SIZE) + offset) as f32;
        let cell_size = CELL_SIZE as f32;
        draw_rectangle(x, y, cell_size, cell_size, BLACK);
    }
}
