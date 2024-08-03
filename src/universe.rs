use fixedbitset::FixedBitSet;
use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: u32, height: u32) -> Self {
        let mut inst = Self {
            width,
            height,
            cells: FixedBitSet::with_capacity((width * height) as usize),
        };

        inst.clear();
        inst
    }

    /**
     * Set all cells to dead.
     */
    pub fn clear(&mut self) {
        self.cells.clear();
    }

    /**
     * Randomize the cell display.
     */
    pub fn randomize(&mut self) {
        for row in 0..self.height {
            for column in 0..self.width {
                let idx = self.get_index(row, column);
                self.cells.set(idx, js_sys::Math::random() < 0.5);
            }
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    pub fn get_cell(&self, row: u32, column: u32) -> bool {
        let idx = self.get_index(row, column);
        self.cells[idx]
    }

    pub fn set_cell(&mut self, row: u32, column: u32, value: bool) {
        let idx = self.get_index(row, column);
        self.cells.set(idx, value);
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx: usize = self.get_index(row, col);
                let cell = self.get_cell(row, col);
                let live_neighbors = self.live_neighbor_count(row, col);

                // println!("cell[{},{}] is initially {:?} and has {} live neighbors", row, col, cell, live_neighbors);

                next.set(idx, match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than 2 live neighbors dies, as if by underpopulation.
                    (true, x)  if x < 2 => false,

                    // Rule 2: Any live cell width 2 or 3 live neighbors lives on.
                    (true, 2) | (true, 3) => true,

                    // Rule 3: Any live cell with more than 3 live neighbors dies, as if by overpopulation.
                    (true, x) if x > 3 => false,

                    // Rule 4: Any dead cell with exactly 3 live neighbors becomes a live cell, as if by reproduction.
                    (false, 3) => true,

                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                });
            }
        }

        self.cells = next;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.height {
            for column in 0..self.width {
                let idx = self.get_index(row, column);
                let symbol = if self.cells[idx] { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}