use std::env;

use rand::{thread_rng, Rng};
use raylib::prelude::*;

#[derive(Clone)]
struct Board {
    size: u32,
    cells: Vec<bool>,
}

impl Board {
    fn new(size: u32) -> Board {
        Board {
            size,
            cells: vec![false; (size * size) as usize],
        }
    }
    fn randomize_cells(&mut self) {
        let mut rng = thread_rng();
        for x in 0..self.size {
            for y in 0..self.size {
                self.set_cell(x, y, rng.gen_bool(0.10));
            }
        }
    }
    fn tick(&mut self) -> u32 {
        let mut cells: Vec<bool> = vec![false; (self.size * self.size) as usize];
        let mut updated_cells = 0;
        for x in 0..self.size {
            for y in 0..self.size {
                let index = get_index(self.size, x, y);
                let cell_alive = self.cells[index];
                let neighbours = self.get_neighbours(x, y);

                if cell_alive && neighbours < 2 {
                    cells[index] = false;
                    updated_cells += 1;
                    continue;
                }
                if cell_alive && neighbours <= 3 {
                    cells[index] = true;
                    continue;
                }
                if cell_alive && neighbours > 3 {
                    cells[index] = false;
                    updated_cells += 1;
                    continue;
                }
                if !cell_alive && neighbours == 3 {
                    cells[index] = true;
                    updated_cells += 1;
                    continue;
                }
            }
        }
        self.cells = cells;
        updated_cells
    }
    fn get_cell(&self, x: u32, y: u32) -> bool {
        if x >= self.size || y >= self.size {
            return false;
        }
        self.cells[get_index(self.size, x, y)]
    }
    fn set_cell(&mut self, x: u32, y: u32, alive: bool) {
        self.cells[get_index(self.size, x, y)] = alive;
    }
    fn get_neighbours(&self, x: u32, y: u32) -> u32 {
        let mut neighbours = 0;
        for i in x.saturating_sub(1)..=x.saturating_add(1) {
            for j in y.saturating_sub(1)..=y.saturating_add(1) {
                if i == x && j == y {
                    continue;
                }
                if self.get_cell(i, j) {
                    neighbours += 1;
                }
            }
        }
        neighbours
    }
}

fn min<T: PartialOrd>(x: T, y: T) -> T {
    if x < y {
        return x;
    }
    y
}

fn get_index(width: u32, x: u32, y: u32) -> usize {
    (y * width + x) as usize
}

fn main() {
    let mut board_size = 1000;
    let args: Vec<String> = env::args().collect();
    if let Some(size) = args.get(1) {
        if let Ok(size_u32) = size.parse::<u32>() {
            board_size = size_u32;
        }
    }
    let mut board = Board::new(board_size);

    let (mut rl, thread) = raylib::init().title("Game of Life").resizable().build();
    rl.set_target_fps(60);

    let mut camera = Camera2D {
        target: Vector2::new(board_size as f32 * 0.5, board_size as f32 * 0.5),
        offset: Vector2::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32),
        rotation: 0.0,
        zoom: 1.0,
    };

    let mut should_tick = false;
    while !rl.window_should_close() {
        let mut bd = rl.begin_drawing(&thread);
        let mut d = bd.begin_mode2D(camera);

        if d.is_window_resized() {
            camera.offset = Vector2::new(
                d.get_screen_width() as f32 * 0.5,
                d.get_screen_height() as f32 * 0.5,
            );
        }

        camera.zoom = min(
            d.get_screen_width() as f32 / board.size as f32,
            d.get_screen_height() as f32 / board.size as f32,
        );

        if d.is_key_pressed(KeyboardKey::KEY_R) {
            board.randomize_cells();
            continue;
        }

        d.clear_background(Color::BLACK);

        for x in 0..board.size {
            for y in 0..board.size {
                if board.get_cell(x, y) {
                    d.draw_pixel(x as i32, y as i32, Color::WHITE);
                }
            }
        }

        // Only calculate a new generation every other frame
        // This is so resizing the window is nice and smooth, without making the simulation too fast
        if should_tick {
            let updated_cells = board.tick();
            if board.size.pow(2) / 100 > updated_cells {
                // Reset cells if >1% of individual cells have updated
                board.randomize_cells();
            }
            should_tick = false;
        } else {
            should_tick = true;
        }
    }
}
