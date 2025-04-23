use nalgebra::DMatrix;

mod tree;
use tree::{Tree, TreeState};


struct Forest {
    size: usize,
    map: DMatrix<Tree>,
}

impl Forest {
    fn new(size: usize) -> Self {
        let map = DMatrix::from_element(size, size, Tree::new_tree());
        Forest { size, map }
    }

    fn set_tree_on_fire(&mut self, width: usize, height: usize) {
        self.map[(width, height)].change_state(TreeState::Burning);
    }

    // fn spread_fire(&mut self, width: usize, height: usize) {
    //     let directions = [(-1, 0), (0, 1), (0, -1), (1, 0)];
    //     self.map[(width,height)].
    // }

    fn update_trees_state(self) {
        self.map.iter().for_each(|x| println!("{}", x.state));
    }
}


fn main() {
    let size = 4;
    let mut forest = Forest::new(size);
    forest.set_tree_on_fire(1, 1);
    forest.set_tree_on_fire(1, 2);

    forest.update_trees_state();
    // println!("{}, Size: {}", forest.map);
}
