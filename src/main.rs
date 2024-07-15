use rand::{thread_rng, Rng};

use raylib::prelude::*;

const SIZE: usize = 100;

#[derive(Clone)]
struct Board {
    cells: [[bool; SIZE]; SIZE],
}

impl Board {
    fn new() -> Board {
        let mut board = Board {
            cells: [[false; SIZE]; SIZE],
        };
        let mut rng = thread_rng();
        for x in 0..SIZE {
            for y in 0..SIZE {
                board.cells[x][y] = rng.gen_bool(0.20);
            }
        }
        return board;
    }
    fn tick(&mut self) {
        let mut cells = [[false; SIZE]; SIZE];
        for x in 0..SIZE {
            for y in 0..SIZE {
                let cell_alive = self.cells[x][y];
                let neighbours = self.get_neighbours(x, y);
                if cell_alive && neighbours < 2 {
                    cells[x][y] = false;
                    continue;
                }
                if cell_alive && neighbours <= 3 {
                    cells[x][y] = true;
                    continue;
                }
                if cell_alive && neighbours > 3 {
                    cells[x][y] = false;
                    continue;
                }
                if !cell_alive && neighbours == 3 {
                    cells[x][y] = true;
                    continue;
                }
            }
        }
        self.cells = cells;
    }
    fn get_cell(&self, x: usize, y: usize) -> bool {
        if x >= SIZE || y >= SIZE {
            return false;
        }
        self.cells[x][y]
    }
    fn get_neighbours(&self, x: usize, y: usize) -> u32 {
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

fn main() {
    let mut board = Board::new();
    let (mut rl, thread) = raylib::init().size(500, 500).title("Game of Life").build();
    rl.set_target_fps(30);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        if d.is_key_pressed(KeyboardKey::KEY_R) {
            board = Board::new();
            continue;
        }

        for x in 0..SIZE {
            for y in 0..SIZE {
                if board.get_cell(x, y) {
                    d.draw_rectangle((x*5) as i32, (y*5) as i32, 5, 5, Color::WHITE);
                }
            }
        }
        board.tick();
    }
}
