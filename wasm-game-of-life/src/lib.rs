mod utils;

extern crate js_sys;
extern crate web_sys;

use std::fmt;
use wasm_bindgen::prelude::*;

// macro_rules! log {
//     ($($t:tt)*) => {
//         web_sys::console::log_1(&format!($($t)*).into());
//     };
// }

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Universe {
    height: u32,
    width: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(height: u32, width: u32) -> Self {
        utils::set_panic_hook();

        let cells = (0..width * height)
            .map(|_| {
                if js_sys::Math::random() < 0.5 {
                    Cell::Dead
                } else {
                    Cell::Alive
                }
            })
            .collect();

        Self {
            height,
            width,
            cells,
        }
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn cells_as_ptr(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn tick(&mut self) {
        let _timer = Timer::new("Universe::tick");

        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let i = self.get_index(row, col);
                let cell = self.cells[i];
                let live_neighbors = self.live_neighbor_count(row, col);

                // log!(
                //     "cell[{}, {}] is initially {:?} and has {} live neighbors",
                //     row,
                //     col,
                //     cell,
                //     live_neighbors
                // );

                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                // if next_cell == cell {
                //     log!("    it is still {:?}", next_cell);
                // } else {
                //     log!("    it becomes {:?}", next_cell);
                // }

                next[i] = next_cell;
            }
        }

        self.cells = next;
    }

    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;

        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_col = (col + delta_col) % self.width;
                let neighbor_row = (row + delta_row) % self.height;
                let i = self.get_index(neighbor_row, neighbor_col);

                count += self.cells[i] as u8;
            }
        }

        count
    }

    pub fn make_all_dead(&mut self) {
        for cell in &mut self.cells {
            *cell = Cell::Dead;
        }
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let i = self.get_index(row, col);
        self.cells[i].toggle();
    }

    fn get_index(&self, row: u32, col: u32) -> usize {
        ((row * self.width) + col) as usize
    }
}

impl Universe {
    pub fn cells_as_slice(&self) -> &[Cell] {
        &self.cells
    }

    pub fn make_cells_alive(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let i = self.get_index(row, col);
            self.cells[i] = Cell::Alive;
        }
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Alive { '◼' } else { '◻' };
                write!(f, "{}", symbol)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        }
    }
}

struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    fn new(name: &'a str) -> Self {
        web_sys::console::time_with_label(name);
        Self { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        web_sys::console::time_end_with_label(self.name);
    }
}
