use rand::prelude::SliceRandom;
use rand::{rng, Rng};
use std::ops::{Index, IndexMut};
use std::slice::Chunks;

#[derive(PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum TreeState {
    None = 0,
    Burned = 1,
    Alive = 2,
}

impl TryFrom<u8> for TreeState {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TreeState::None),
            1 => Ok(TreeState::Burned),
            2 => Ok(TreeState::Alive),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct Forest {
    pub size: (usize, usize), // (rows, cols)
    pub trees_last_burned_positions: Vec<(usize, usize)>,
    pub data: Vec<TreeState>,
    pub burned: usize,
}

impl Forest {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            burned: 0,
            trees_last_burned_positions: Vec::new(),
            size: (rows, cols),
            data: vec![TreeState::None; rows * cols],
        }
    }

    pub fn create_alive_trees2(&mut self, persent: usize) {
        let total_cells = self.size.0 * self.size.1;
        let target_alive = (persent as u64 * total_cells as u64 / 100) as usize;
        let mut alive_count = 0;
        let mut rng = rand::rng();
        let mut remaining_cells = total_cells;

        for row in 0..self.size.0 {
            for col in 0..self.size.1 {
                if alive_count >= target_alive {
                    return;
                }
                let probability = (target_alive - alive_count) as f64 / remaining_cells as f64;
                if rng.random_bool(probability) {
                    self[row][col] = TreeState::Alive;
                    alive_count += 1;
                }
                remaining_cells -= 1;
            }
        }
    }

    pub fn create_alive_trees(&mut self, persent: usize) {
        let mut alive_count = 0;
        let alive_needed = (self.size.0 * self.size.1 * persent) / 100;

        'out: for row in self.iter_mut() {
            for tree_state in row.iter_mut() {
                *tree_state = TreeState::Alive;
                alive_count += 1;
                if alive_count >= alive_needed {
                    break 'out;
                }
            }
        }
        self.data.shuffle(&mut rng());
    }

    pub fn fire_random_tree(&mut self) {
        let (row, col) = self.size;
        loop {
            let random_row = rand::random_range(0..row);
            let random_col = rand::random_range(0..col);
            if self[random_row][random_col] == TreeState::Alive {
                self[random_row][random_col] = TreeState::Burned;
                self.trees_last_burned_positions
                    .push((random_row, random_col));
                self.burned += 1;
                return;
            }
        }
    }

    pub fn spread_fire(&mut self) {
        let (rows, cols) = self.size;
        let mut capacity = self.trees_last_burned_positions.len() * 2;
        if capacity >= self.data.len() {
            capacity = self.data.len()
        }
        let mut new_burning_positions: Vec<(usize, usize)> = Vec::with_capacity(capacity);
        for &(row, col) in self.trees_last_burned_positions.iter() {
            // top
            if row > 0 {
                let row = row - 1;
                let index = self.get_index(row, col);
                if self.data[index] == TreeState::Alive {
                    self.data[index] = TreeState::Burned;
                    self.burned += 1;
                    new_burning_positions.push((row, col));
                }
            }
            // bottom
            if row < rows - 1 {
                let row = row + 1;
                let index = self.get_index(row, col);
                if self.data[index] == TreeState::Alive {
                    self.data[index] = TreeState::Burned;
                    self.burned += 1;
                    new_burning_positions.push((row, col));
                }
            }
            // left
            if col > 0 {
                let col = col - 1;
                let index = self.get_index(row, col);
                if self.data[index] == TreeState::Alive {
                    self.data[index] = TreeState::Burned;
                    self.burned += 1;
                    new_burning_positions.push((row, col));
                }
            }
            // right
            if col < cols - 1 {
                let col = col + 1;
                let index = self.get_index(row, col);
                if self.data[index] == TreeState::Alive {
                    self.data[index] = TreeState::Burned;
                    self.burned += 1;
                    new_burning_positions.push((row, col));
                }
            }
        }
        self.trees_last_burned_positions = new_burning_positions;
    }

    pub fn get_index(&self, row: usize, col: usize) -> usize {
        let cols = self.size.1;
        row * cols + col

    }

    pub fn iter(&self) -> Chunks<TreeState> {
        self.data.chunks(self.size.1)
    }

    pub fn iter_mut(&mut self) -> std::slice::ChunksMut<TreeState> {
        self.data.chunks_mut(self.size.1)
    }
}

impl Index<usize> for Forest {
    type Output = [TreeState];
    fn index(&self, row: usize) -> &Self::Output {
        let cols = self.size.1;
        let start = row * cols;
        &self.data[start..start + cols]
    }
}

impl IndexMut<usize> for Forest {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        let cols = self.size.1;
        let start = row * cols;
        &mut self.data[start..start + cols]
    }
}
