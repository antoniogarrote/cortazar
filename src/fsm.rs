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

    pub fn next_state(&self, from:&String, by:&String) -> Option<&String> {
        self.transitions.get(from).and_then(|transitions| transitions.get(by))
    }
}

impl fmt::Display for FSM {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = write!(f,"\nFSM[\n - Initial state: {}\n - Transitions:",self.initial_state);
        for from in self.transitions.keys() {
            for by in self.transitions.get(from).unwrap().keys() {
                let to = self.transitions.get(from).unwrap().get(by).unwrap();
                let _ =  write!(f, "\n   * {} -{}-> {}", from, by, to);
            }
        }
        write!(f,"\n]\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_a_fsm() {
        let fsm = FSM::new("initial_state".to_string());
        assert_eq!(fsm.transitions.len(),0);
        assert_eq!(fsm.initial_state, "initial_state");
    }

    #[test]
    fn add_transitions() {
        let check_fsm_transition = |fsm:&FSM| {
            let from_a = fsm.transitions.get("a");
            assert!(from_a.is_some());
            let by_1 = from_a.unwrap().get("1");
            assert!(by_1.is_some());
            assert_eq!(by_1.unwrap(), "b");
        };

        let mut fsm = FSM::new("initial_state".to_string());
        fsm.add_transition("a".to_string(),
                           "b".to_string(),
                           "1".to_string());
        check_fsm_transition(&fsm);

        fsm.add_transition("a".to_string(),
                           "b".to_string(),
                           "1".to_string());
        check_fsm_transition(&fsm);
    }

    #[test]
    fn check_transitions() {
        let mut fsm = FSM::new("initial_state".to_string());
        fsm.add_transition("a".to_string(),"b".to_string(),"1".to_string());
        fsm.add_transition("a".to_string(),"c".to_string(),"2".to_string());
        fsm.add_transition("b".to_string(),"d".to_string(),"3".to_string());

        assert_eq!(fsm.next_state(&"a".to_string(), &"1".to_string()), Some(&"b".to_string()));
        assert_eq!(fsm.next_state(&"a".to_string(), &"3".to_string()), None);
    }
}
