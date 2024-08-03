use crate::universe::Universe;
use fixedbitset::FixedBitSet;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, PartialEq, Eq)]
pub struct Pattern {
    width: usize,
    height: usize,
    data: FixedBitSet,
}

#[wasm_bindgen]
impl Pattern {
    pub fn from_universe(universe: &Universe, row: usize, column: usize, width: usize, height: usize) -> Pattern {
        let mut data = FixedBitSet::with_capacity((width * height) as usize);
        for pattern_row in 0..height {
            for pattern_column in 0..width {
                let universe_row = row + pattern_row;
                let universe_col = column + pattern_column;
                let value = universe.get_cell(universe_row as u32, universe_col as u32);
                data.set(pattern_row * width + pattern_column, value);
            }
        }
        Self { width, height, data }
    }

    pub fn from_str(text: &str) -> Self {
        let lines = text.trim().lines();
        let height = lines.clone().count();

        let width = lines
            .clone()
            .map(|line| line.trim().chars().count())
            .max()
            .unwrap_or(0);

        let mut data = FixedBitSet::with_capacity((width * height) as usize);

        for (row, line) in lines.enumerate() {
            for (column, c) in line.trim().chars().enumerate() {
                let idx = row * width + column;
                match c {
                    '0' => data.set(idx, false),
                    '1' => data.set(idx, true),
                    _ => panic!("Invalid character in pattern"),
                }
            }
        }

        Self { width, height, data }
    }

    pub fn to_str(&self) -> String {
        let mut text: String = "".to_owned();
        for row in 0..self.height {
            for column in 0..self.width {
                let idx = row * self.width + column;
                match self.data[idx] {
                    true => text += "1",
                    false => text += "0",
                }
            }
            text += "\n";
        }
        text.trim().to_string()
    }

    pub fn get(&self, row: usize, column: usize) -> bool {
        self.data[row * self.width + column]
    }

    pub fn place(self, universe: &mut Universe, row: usize, column: usize) {
        for pattern_row in 0..self.height {
            for pattern_column in 0..self.width {
                let universe_row = row + pattern_row;
                let universe_col = column + pattern_column;
                let value = self.get(pattern_row, pattern_column);
                universe.set_cell(universe_row as u32, universe_col as u32, value);
            }
        }
    }
}
