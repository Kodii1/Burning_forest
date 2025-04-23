#[derive(Clone, PartialEq, Debug)]

pub enum TreeState {
    Normal,
    Burning,
    Burned,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Tree {
    pub state: TreeState,
}

impl Tree {
    pub fn new_tree() -> Self {
        Tree {
            state: TreeState::Normal,
        }
    }

    pub fn change_state(&mut self, new_state: TreeState) {
        self.state = new_state;
    }
}

impl std::fmt::Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write! {f,"{}", self.state}
    }
}

impl std::fmt::Display for TreeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state_str = match self {
            TreeState::Normal => "Normal",
            TreeState::Burning => "Burning",
            TreeState::Burned => "Burned",
        };
        write!(f, "{}", state_str)
    }
}
