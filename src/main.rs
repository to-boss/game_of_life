pub mod board;

use board::Board;
use macroquad::prelude::*;

const BOARD_SIZE: usize = 40;
const CELL_SIZE: usize = 16;

fn window_conf() -> Conf {
    Conf {
        window_title: "Game of Life".to_owned(),
        window_width: ((BOARD_SIZE * CELL_SIZE) + 80) as i32,
        window_height: ((BOARD_SIZE * CELL_SIZE) + 80) as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
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
        if is_key_pressed(KeyCode::Space) {
            ticking = !ticking;
        }

        if is_key_pressed(KeyCode::S) {
            board.tick();
        }

        if is_key_pressed(KeyCode::R) {
            board.reset();
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            if play.contains(Vec2::new(mouse_x, mouse_y)) {
                ticking = !ticking;
            }
            if reset.contains(Vec2::new(mouse_x, mouse_y)) {
                board.reset();
            }
            if let Some((x, y)) = board.mouse_to_cell_position() {
                board.switch_cell(x, y);
            }
        }

        let rect_color = if ticking { RED } else { GREEN };

        clear_background(WHITE);
        draw_rectangle(play.x, play.y, play.w, play.h, rect_color);
        draw_rectangle(reset.x, reset.y, reset.w, reset.h, BLACK);
        draw_debug_text();
        draw_controls();
        board.draw();

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

fn draw_controls() {
    let offset = 40. + 40. + 10. + 40. + 10. + 10.;
    draw_text("Space: Play/Pause", offset, 10., 11., BLACK);
    draw_text("S: Single Step", offset, 20., 11., BLACK);
    draw_text("R: Reset Board", offset, 30., 11., BLACK);
}

fn draw_debug_text() {
    let x_pos = screen_width() - 100.;

    let fps_text = format!("FPS: {}", get_fps());
    draw_text(&fps_text, x_pos, 10., 11., BLACK);

    let mouse_text = format!("Mouse: ({}, {})", mouse_position().0, mouse_position().1);
    draw_text(&mouse_text, x_pos, 20., 11., BLACK);
}
