use minifb::{Window, WindowOptions, Key};
use std::time::Duration;

mod framebuffer;

use crate::framebuffer::Framebuffer;

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 80;
    let framebuffer_height = 60;

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Conway's Game of Life",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    framebuffer.set_background_color(0x000000);
    framebuffer.set_current_color(0xFFFFFF);

    let mut grid = vec![vec![false; framebuffer_width]; framebuffer_height];
    initialize_grid(&mut grid);

    let frame_delay = Duration::from_millis(100);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        framebuffer.draw_grid(&grid);
        grid = update_grid(&grid);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}

fn initialize_grid(grid: &mut Vec<Vec<bool>>) {
    grid[11][12] = true;
    grid[11][13] = true;
    grid[12][12] = true;
    grid[12][13] = true;
    grid[13][12] = true;
    grid[13][14] = true;
    grid[14][13] = true;
    grid[15][13] = true;
}

fn update_grid(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let height = grid.len();
    let width = grid[0].len();
    let mut new_grid = vec![vec![false; width]; height];

    for y in 0..height {
        for x in 0..width {
            let live_neighbors = count_live_neighbors(grid, x, y);
            if grid[y][x] {
                new_grid[y][x] = live_neighbors == 2 || live_neighbors == 3;
            } else {
                new_grid[y][x] = live_neighbors == 3;
            }
        }
    }

    new_grid
}

fn count_live_neighbors(grid: &Vec<Vec<bool>>, x: usize, y: usize) -> usize {
    let mut count = 0;
    let height = grid.len();
    let width = grid[0].len();

    for dy in [-1, 0, 1].iter().cloned() {
        for dx in [-1, 0, 1].iter().cloned() {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                if grid[ny as usize][nx as usize] {
                    count += 1;
                }
            }
        }
    }

    count
}
