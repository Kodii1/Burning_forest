use rand::prelude::SliceRandom;
use rand::rng;
use std::ops::{Index, IndexMut};
use std::slice::Chunks;

#[derive(PartialEq, Clone, Debug)]
#[repr(u8)]
pub enum TreeState {
    None = 0,
    Burned = 1,
    Alive = 2,
}

pub struct Forest {
    pub size: (usize, usize), // (rows, cols)
    pub trees_last_burned_positions: Vec<(usize, usize)>,
    pub data: Vec<TreeState>,
    pub burned: usize,
    pub alive: f32,
}

impl Forest {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            burned: 0,
            trees_last_burned_positions: Vec::new(),
            size: (rows, cols),
            data: vec![TreeState::None; rows * cols],
            alive: 0.0,
        }
    }

    pub fn create_alive_trees(&mut self, persent: f32) {
        let mut alive_count = 0.0;
        let alive_needed = ((self.size.0 * self.size.1) as f32 * persent) / 100.0;

        'out: for row in self.iter_mut() {
            for tree_state in row.iter_mut() {
                *tree_state = TreeState::Alive;
                alive_count += 1.0;
                if alive_count >= alive_needed {
                    break 'out;
                }
            }
        }
        self.alive = alive_count;
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

    ///Return index of tree in Vec
    pub fn get_index(&self, row: usize, col: usize) -> usize {
        let cols = self.size.1;
        row * cols + col
    }

    ///Iter for Vec with logic like Vec<Vec>
    pub fn iter(&self) -> Chunks<TreeState> {
        self.data.chunks(self.size.1)
    }

    ///Iter_mut for Vec with logic like Vec<Vec>
    pub fn iter_mut(&mut self) -> std::slice::ChunksMut<TreeState> {
        self.data.chunks_mut(self.size.1)
    }
}

///Logic like Vec<Vec>
impl Index<usize> for Forest {
    type Output = [TreeState];
    fn index(&self, row: usize) -> &Self::Output {
        let cols = self.size.1;
        let start = row * cols;
        &self.data[start..start + cols]
    }
}

///Logic like Vec<Vec>
impl IndexMut<usize> for Forest {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        let cols = self.size.1;
        let start = row * cols;
        &mut self.data[start..start + cols]
    }
}
