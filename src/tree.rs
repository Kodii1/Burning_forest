#[derive(Debug, Clone, PartialEq)]
pub enum TreeState {
    None,
    Burned,
    Alive,
}

#[derive(Debug, Clone)]
pub struct Tree {
    pub state: TreeState,
}


