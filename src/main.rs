use macroquad::prelude::*;

const BOARD_SIZE: usize = 40;
const CELL_SIZE: usize = 16;

struct Board {
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

    pub fn draw(&self) {
        self.draw_grid();
        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                if self.cells[x][y].is_some() {
                    draw_cell(x, y, self.offset);
                }
            }
        }
    }

    pub fn mouse_to_cell_position(&self) -> (usize, usize) {
        let (x, y) = mouse_position();
        let size_f32 = CELL_SIZE as f32;
        let offset = self.offset as f32;
        let x = ((x - offset) / size_f32) as usize;
        let y = ((y - offset) / size_f32) as usize;
        (x, y)
    }

    fn draw_grid(&self) {
        let max = ((BOARD_SIZE * CELL_SIZE) + self.offset) as f32;
        let offset = self.offset as f32;

        for i in 0..(BOARD_SIZE + 1) as usize {
            let i = ((i * CELL_SIZE) + self.offset) as f32;
            draw_line(i, offset, i, max, 0.5, BLACK);
            draw_line(offset, i, max, i, 0.5, BLACK);
        }
    }
}

#[macroquad::main("Game of Life")]
async fn main() {
    let mut board = Board::init();
    let mut ticking = false;
    let mut time_passed = 0.;

    let play = Rect {
        x: 40.,
        y: 10.,
        w: 40.,
        h: 20.,
    };
    let reset = Rect {
        x: 40. + 40. + 10.,
        y: 10.,
        w: 40.,
        h: 20.,
    };

    loop {
        clear_background(WHITE);

        let rect_color = if ticking { RED } else { GREEN };
        draw_rectangle(play.x, play.y, play.w, play.h, rect_color);
        draw_rectangle(reset.x, reset.y, reset.w, reset.h, BLACK);

        board.draw();
        draw_debug_text();

        if is_key_pressed(KeyCode::Space) {
            board.tick();
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            if play.contains(Vec2::new(mouse_x, mouse_y)) {
                ticking = !ticking;
            }

            if reset.contains(Vec2::new(mouse_x, mouse_y)) {
                board.reset();
            }

            let (x, y) = board.mouse_to_cell_position();
            if x < BOARD_SIZE && y < BOARD_SIZE {
                board.switch_cell(x, y);
            }
        }

        if time_passed < 1. / 60. {
            time_passed += get_frame_time();
        } else {
            time_passed = 0.;
            if ticking {
                board.tick();
            }
        }

        next_frame().await;
    }
}

fn draw_cell(x: usize, y: usize, offset: usize) {
    let x = ((x * CELL_SIZE) + offset) as f32;
    let y = ((y * CELL_SIZE) + offset) as f32;
    let cell_size = CELL_SIZE as f32;
    draw_rectangle(x, y, cell_size, cell_size, BLACK);
}

fn draw_debug_text() {
    draw_text(
        format!("FPS: {}", get_fps()).as_str(),
        screen_width() - 44.,
        10.,
        11.,
        BLACK,
    );

    draw_text(
        format!("Mouse: ({}, {})", mouse_position().0, mouse_position().1).as_str(),
        screen_width() - 88.,
        20.,
        11.,
        BLACK,
    );
}
