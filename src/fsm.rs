use std::collections::HashMap;
use std::fmt;

pub struct FSM {
    pub initial_state: String,
    pub transitions: HashMap<String,HashMap<String,String>>
}

impl FSM {

    pub fn new(initial_state:String) -> FSM {
        FSM { initial_state: initial_state, transitions: HashMap::new() }
    }

    pub fn build(initial_state:String, transitions_list:& [(String,String,String)]) -> FSM {
        let mut fsm = FSM::new(initial_state);
        for transition in transitions_list {
            match transition.clone() {
                (from,by,to) => fsm.add_transition(from,to,by)
            }
        }
        fsm
    }

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

impl fmt::Display for FSM {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = write!(f,"\nFSM[\n - Initial state: {}\n - Transitions:",self.initial_state);
        for from in self.transitions.keys() {
            for by in self.transitions.get(from).unwrap().keys() {
                let to = self.transitions.get(from).unwrap().get(by).unwrap();
                let _ = write!(f, "\n   * {} -{}-> {}", from, by, to);
            }
        }
        write!(f,"\n]\n")
    }
}
