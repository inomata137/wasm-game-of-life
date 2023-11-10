mod utils;

use wasm_bindgen::prelude::*;
use std::fmt;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
  Dead = 0,
  Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
  width: u32,
  height: u32,
  cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
  fn get_index(&self, row: u32, column: u32) -> usize {
    (row * self.width + column) as usize
  }

  fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
    let mut count: u8 = 0;
    for dr in [self.height - 1, 0, 1] {
      for dc in [self.width - 1, 0, 1] {
        if dr == 0 && dc == 0 {
          continue;
        }
        let row_idx = (row + dr) % self.height;
        let col_idx = (column + dc) % self.width;
        let idx = self.get_index(row_idx, col_idx);
        count += self.cells[idx] as u8;
      }
    }
    count
  }

  pub fn tick(&mut self) {
    let mut next = self.cells.clone();

    for row in 0..self.height {
      for col in 0..self.width {
        let idx = self.get_index(row, col);
        let live_neighbors = self.live_neighbor_count(row, col);

        let next_cell = match (self.cells[idx], live_neighbors) {
            (Cell::Alive, x) if x < 2 => Cell::Dead,
            (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
            (Cell::Alive, x) if x > 3 => Cell::Dead,
            (Cell::Dead, 3) => Cell::Alive,
            (otherwise, _) => otherwise,
        };

        next[idx] = next_cell;
      }
    }

    self.cells = next;
  }

  pub fn new() -> Universe {
    let width = 512;
    let height = 256;
    let cells = (0..width * height)
      .map(|i| {
        if i % 2 == 0 || i % 7 == 0 {
          Cell::Alive
        } else {
          Cell::Dead
        }
      })
      .collect();

    Universe {
      width,
      height,
      cells
    }
  }

  pub fn render(&self) -> String {
    self.to_string()
  }

  pub fn width(&self) -> u32 {
    self.width
  }

  pub fn height(&self) -> u32 {
    self.height
  }

  pub fn cells(&self) -> *const Cell {
    self.cells.as_ptr()
  }
}

impl fmt::Display for Universe {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      for line in self.cells.as_slice().chunks(self.width as usize) {
        for &cell in line {
          let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
          write!(f, "{}", symbol).unwrap();
        }
        write!(f, "\n").unwrap();
      }
      Ok(())
  }
}
