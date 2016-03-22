use std::collections::HashMap;

pub struct FSM {
    initial_state: String,
    transitions: HashMap<String,HashMap<String,String>>
}

pub fn new(initial_state:String) -> FSM {
    FSM { initial_state: initial_state, transitions: HashMap::new() }
}

impl FSM {
    pub fn add_transition(&mut self, from:String, to:String, by:String) {
        if self.transitions.get(&from).is_some() {
            self.transitions.get_mut(&from).unwrap().insert(by, to);
        } else {
            let mut new_transitions = HashMap::new();
            new_transitions.insert(by, to);
            self.transitions.insert(from, new_transitions);
        }
    }
}
