use raylib::prelude::*;

#[derive(Clone)]
struct Board {
    width: u32,
    height: u32,
    cells: Vec<u8>,
}

impl Board {
    fn new(width: u32, height: u32) -> Board {
        Board {
            width,
            height,
            cells: vec![0; (width * height) as usize],
        }
    }
    fn randomize_cells(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                if fastrand::u8(0..10) == 0 {
                    self.set_cell(x, y, 1);
                }
            }
        }
    }
    fn tick(&mut self) -> u32 {
        let mut cells: Vec<u8> = vec![0; (self.width * self.height) as usize];
        let mut updated_cells = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                let index = get_index(self.width, x, y);
                let cell = self.cells[index];
                let neighbours = self.get_neighbours(x, y);

                if cell > 0 && neighbours < 2 {
                    cells[index] = 0;
                    updated_cells += 1;
                    continue;
                }
                if cell > 0 && neighbours <= 3 {
                    cells[index] = min(cell + 1, 6);
                    continue;
                }
                if cell > 0 && neighbours > 3 {
                    cells[index] = 0;
                    updated_cells += 1;
                    continue;
                }
                if cell == 0 && neighbours == 3 {
                    cells[index] = 1;
                    updated_cells += 1;
                    continue;
                }
            }
        }
        self.cells = cells;
        updated_cells
    }
    fn get_cell(&self, x: u32, y: u32) -> u8 {
        if x >= self.width || y >= self.height {
            return 0;
        }
        self.cells[get_index(self.width, x, y)]
    }
    fn set_cell(&mut self, x: u32, y: u32, cell: u8) {
        self.cells[get_index(self.width, x, y)] = cell;
    }
    fn get_neighbours(&self, x: u32, y: u32) -> u32 {
        let mut neighbours = 0;
        for i in x.saturating_sub(1)..=x.saturating_add(1) {
            for j in y.saturating_sub(1)..=y.saturating_add(1) {
                if i == x && j == y {
                    continue;
                }
                if self.get_cell(i, j) > 0 {
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

fn get_color(cell: u8) -> Color {
    match cell {
        1 => Color::RED,
        2 => Color::ORANGERED,
        3 => Color::ORANGE,
        4 => Color::YELLOW,
        5 => Color::DARKGRAY,
        6 => Color::GRAY,
        _ => Color::BLACK,
    }
}

fn main() {
    let (mut rl, thread) = raylib::init().title("Game of Life").resizable().build();
    rl.set_target_fps(60);

    let mut board = Board::new(
        rl.get_screen_width() as u32 / 2,
        rl.get_screen_height() as u32 / 2,
    );

    let mut camera = Camera2D {
        target: Vector2::new(board.width as f32 * 0.5, board.height as f32 * 0.5),
        offset: Vector2::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32),
        rotation: 0.0,
        zoom: 1.0,
    };

    let mut should_tick = false;
    let mut is_paused = false;

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
            d.get_screen_width() as f32 / board.width as f32,
            d.get_screen_height() as f32 / board.height as f32,
        );

        if d.is_key_pressed(KeyboardKey::KEY_SPACE) {
            is_paused = !is_paused
        }
        if d.is_key_pressed(KeyboardKey::KEY_TAB) && is_paused {
            board.tick();
        }

        if d.is_key_pressed(KeyboardKey::KEY_R) || d.is_window_resized() {
            board.width = d.get_screen_width() as u32 / 2;
            board.height = d.get_screen_height() as u32 / 2;
            board.cells = vec![0; (board.width * board.height) as usize];
            camera.target = Vector2::new(board.width as f32 * 0.5, board.height as f32 * 0.5);
            board.randomize_cells();
            continue;
        }

        d.clear_background(Color::BLACK);

        for x in 0..board.width {
            for y in 0..board.height {
                let cell = board.get_cell(x, y);
                if cell > 0 {
                    d.draw_pixel(x as i32, y as i32, get_color(cell));
                }
            }
        }

        if d.get_time() < 5.0 {
            let text = "Press R to reload cells";
            let w = d.measure_text(text, 5);
            d.draw_text(
                text,
                board.width as i32 / 2 - w / 2,
                board.height as i32 / 2 - 10,
                5,
                Color::GRAY,
            );
        }

        // Only calculate a new generation every other frame
        // This is so resizing the window is nice and smooth, without making the simulation too fast
        if should_tick && !is_paused {
            let updated_cells = board.tick();
            if board.width * board.height / 100 > updated_cells {
                // Reset cells if >1% of individual cells have updated
                board.randomize_cells();
            }
            should_tick = false;
        } else {
            should_tick = true;
        }
    }
}
