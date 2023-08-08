use crate::{TreeItem, TreeState, Node};

pub struct StatefulTree {
    pub state: TreeState,
    pub items: Vec<TreeItem>,
}

impl StatefulTree {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            state: TreeState::default(),
            items: Vec::new(),
        }
    }

    pub fn with_items(items: Vec<TreeItem>) -> Self {
        Self {
            state: TreeState::default(),
            items,
        }
    }
    pub fn with_items_and_state(items: Vec<TreeItem>, state: TreeState) -> Self {
        Self {
            state,
            items,
        }
    }

    pub fn first(&mut self) {
        self.state.select_first();
    }

    pub fn last(&mut self) {
        self.state.select_last(&self.items);
    }

    pub fn down(&mut self) {
        self.state.key_down(&self.items);
    }

    pub fn up(&mut self) {
        self.state.key_up(&self.items);
    }

    pub fn left(&mut self) {
        self.state.key_left();
    }

    pub fn right(&mut self) {
        self.state.key_right();
    }

    pub fn toggle(&mut self) {
        self.state.toggle_selected();
    }
    pub fn get_node(&self) -> Option<Node> {
        let pos = &self.state.selected;
        let mut item0 = &self.items[pos[0]];
        for idx in pos[1..].iter() {
            match item0.child(*idx) {
                Some(e) => item0 = e,
                None => return None,

            }
        };
        Some(item0.text.clone())
    }
    pub fn get_state(&self) -> &TreeState {
        &self.state
    }
}
