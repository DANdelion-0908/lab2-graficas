use minifb::{Window, WindowOptions, Key};
use std::time::Duration;

mod framebuffer;

use crate::framebuffer::Framebuffer;

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 70;
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
    let glider: Vec<(usize, usize)> = vec![
        (1, 0), (2, 1), (0, 2), (1, 2), (2, 2)
    ];

    let lwss: Vec<(usize, usize)> = vec![
        (0, 1), (1, 1), (2, 1), (3, 1), (4, 1),
        (4, 2), (0, 3), (1, 3), (2, 3), (3, 3),
        (4, 3), (4, 4), (3, 2), (3, 4), (2, 4)
    ];

    let blinker: Vec<(usize, usize)> = vec![
        (1, 0), (1, 1), (1, 2)
    ];

    let toad: Vec<(usize, usize)> = vec![
        (1, 1), (2, 1), (3, 1),
        (0, 2), (1, 2), (2, 2)
    ];

    let beacon: Vec<(usize, usize)> = vec![
        (1, 1), (2, 1), (1, 2), (2, 2),
        (3, 3), (4, 3), (3, 4), (4, 4)
    ];

    let pulsar: Vec<(usize, usize)> = vec![
        (1, 3), (1, 4), (1, 5), (1, 6),
        (2, 1), (2, 7), (3, 1), (3, 7),
        (4, 1), (4, 7), (5, 1), (5, 7),
        (6, 1), (6, 7), (6, 3), (6, 4),
        (6, 5), (6, 6), (7, 3), (7, 4),
        (7, 5), (7, 6)
    ];

    let pentadecathlon: Vec<(usize, usize)> = vec![
        (1, 3), (2, 2), (2, 3), (2, 4), (3, 3),
        (4, 2), (4, 3), (4, 4), (5, 3), (6, 3)
    ];

    let gosper_glider_gun: Vec<(usize, usize)> = vec![
        (1, 5), (2, 5), (1, 6), (2, 6),
        (11, 5), (11, 6), (11, 7), (12, 4), (13, 3),
        (14, 3), (15, 3), (16, 4), (16, 5), (16, 6),
        (15, 7), (14, 8), (13, 9), (12, 9), (11, 8),
        (9, 6), (9, 7), (8, 5), (7, 5), (6, 6),
        (8, 6), (7, 6), (6, 7), (6, 8), (7, 8),
        (8, 8)
    ];

    let r_pentomino: Vec<(usize, usize)> = vec![
        (1, 1), (2, 1), (1, 2), (2, 2), (2, 3)
    ];

    let diehard: Vec<(usize, usize)> = vec![
        (2, 2), (2, 3), (3, 2), (4, 1), (4, 2),
        (4, 3), (5, 2), (6, 2)
    ];

    let acorn: Vec<(usize, usize)> = vec![
        (2, 2), (2, 3), (3, 3), (4, 2), (5, 2),
        (5, 3), (6, 3), (4, 4)
    ];

    let block: Vec<(usize, usize)> = vec![
        (1, 1), (1, 2), (2, 1), (2, 2)
    ];
    
    let beehive: Vec<(usize, usize)> = vec![
        (1, 2), (2, 1), (2, 3), (3, 2)
    ];
    
    let loaf: Vec<(usize, usize)> = vec![
        (1, 1), (2, 1), (3, 2), (1, 3), (2, 3),
        (3, 4), (4, 2)
    ];
    
    let boat: Vec<(usize, usize)> = vec![
        (1, 1), (2, 1), (1, 2), (2, 2), (3, 2)
    ];
    
    let tub: Vec<(usize, usize)> = vec![
        (1, 1), (1, 2), (2, 1), (2, 2), (2, 3), (3, 2)
    ];
    
    let patterns = vec![
        (glider, 10, 10),
        (lwss, 50, 15),
        (blinker, 20, 20),
        (toad, 30, 30),
        (pulsar, 10, 40),
        (beacon, 10, 60),
        (pentadecathlon, 50, 20),
        (gosper_glider_gun, 30, 30),
        (diehard, 40, 40),
        (acorn, 50, 10),
        (block, 10, 50),
        (beehive, 20, 10),
        (r_pentomino, 40, 50),
        (loaf, 10, 51),
        (boat, 11, 41),
        (tub, 50, 20)
    ];

    for (pattern, x_offset, y_offset) in patterns {
        for (x, y) in pattern {
            let grid_x = x + x_offset;
            let grid_y = y + y_offset;
            if grid_x < grid.len() && grid_y < grid[0].len() {
                grid[grid_x][grid_y] = true;
            }
        }
    }

    // let glider_x_offset = 20;
    // let glider_y_offset = 40;
    // for (x, y) in glider {
    //     let grid_x = glider_x_offset + x;
    //     let grid_y = glider_y_offset + y;
    //     if grid_x < grid.len() && grid_y < grid[0].len() {
    //         grid[grid_x][grid_y] = true;
    //     }
    // }
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
