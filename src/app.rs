use crate::tree::{Tree, TreeState};

pub enum CurrentScreen {
    Main,
    Exiting,
}

pub struct App {
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered
    pub forest: Vec<Vec<Tree>>,
    pub size: (usize, usize),
    pub percent: usize,
}

impl App {
    pub fn new(percent: usize, rows: usize, cols: usize) -> App {
        App {
            current_screen: CurrentScreen::Main,
            size: (rows, cols),
            forest: vec![
                vec![
                    Tree {
                        state: TreeState::None
                    };
                    cols
                ];
                rows
            ],
            percent,
        }
    }

    pub fn create_forest(forest: &mut Vec<Vec<Tree>>, percent: usize, rows: usize, columns: usize) {
        let total_cells = rows * columns;
        let max_alive = (total_cells * percent) / 100;


        let mut positions = Vec::new();
        for y in 0..rows {
            for x in 0..columns {
                positions.push((y, x));
            }
        }

        let mut chosen_positions = Vec::new();

        while chosen_positions.len() < max_alive {
            let rand_index = rand::random_range(0..positions.len());
            let position = positions.swap_remove(rand_index); 
            chosen_positions.push(position);
        }

        for (y, x) in chosen_positions {
            forest[y][x].state = TreeState::Alive;
        }
    }

pub fn spread_fire(forest: &mut Vec<Vec<Tree>>) {
    let estimated_size = forest.len() * forest[0].len() / 2;
    let mut burned_positions = Vec::with_capacity(estimated_size);

    for (y, row) in forest.iter().enumerate() {
        for (x, tree) in row.iter().enumerate() {
            if tree.state == TreeState::Burned {
                burned_positions.push((x, y));
            }
        }
    }

    for (x, y) in burned_positions {
        if x > 0 && forest[y][x - 1].state == TreeState::Alive {
            forest[y][x - 1].state = TreeState::Burned;
        }
        if x < forest[0].len() - 1 && forest[y][x + 1].state == TreeState::Alive {
            forest[y][x + 1].state = TreeState::Burned;
        }
        if y > 0 && forest[y - 1][x].state == TreeState::Alive {
            forest[y - 1][x].state = TreeState::Burned;
        }
        if y < forest.len() - 1 && forest[y + 1][x].state == TreeState::Alive {
            forest[y + 1][x].state = TreeState::Burned;
        }
    }
}

    pub fn fire_random_tree(forest: &mut Vec<Vec<Tree>>) {
        let rows = forest.len();
        let columns = forest[0].len();
        loop {
            let random_row: usize = rand::random_range(0..rows - 1);
            let random_column: usize = rand::random_range(0..columns - 1);
            match forest
                .get_mut(random_row)
                .and_then(|row| row.get_mut(random_column))
            {
                Some(tree) => match tree.state {
                    TreeState::Alive => {
                        tree.state = TreeState::Burned;
                        break;
                    }
                    _ => {}
                },
                None => continue,
            }
        }
    }
}
