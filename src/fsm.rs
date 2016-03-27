use std::collections::HashMap;
use std::fmt;

pub struct FSM {
    pub initial_state: String,
    pub actions: HashMap<String,HashMap<String,String>>
}

impl FSM {

    pub fn new(initial_state:String) -> FSM {
        FSM { initial_state: initial_state, actions: HashMap::new() }
    }

    pub fn build(initial_state:String, actions_list:& [(String,String,String)]) -> FSM {
        let mut fsm = FSM::new(initial_state);
        for action in actions_list {
            match action.clone() {
                (from,by,to) => fsm.add_action(from,to,by)
            }
        }
        fsm
    }

    pub fn add_action(&mut self, from:String, to:String, by:String) {
        if self.actions.get(&from).is_some() {
            self.actions.get_mut(&from).unwrap().insert(by, to);
        } else {
            let mut new_actions = HashMap::new();
            new_actions.insert(by, to);
            self.actions.insert(from, new_actions);
        }
    }

    pub fn next_state(&self, from:&String, by:&String) -> Option<&String> {
        self.actions.get(from).and_then(|actions| actions.get(by))
    }
}

impl fmt::Display for FSM {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = write!(f,"\nFSM[\n - Initial state: {}\n - Actions:",self.initial_state);
        for from in self.actions.keys() {
            for by in self.actions.get(from).unwrap().keys() {
                let to = self.actions.get(from).unwrap().get(by).unwrap();
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
        assert_eq!(fsm.actions.len(),0);
        assert_eq!(fsm.initial_state, "initial_state");
    }

    #[test]
    fn add_actions() {
        let check_fsm_action = |fsm:&FSM| {
            let from_a = fsm.actions.get("a");
            assert!(from_a.is_some());
            let by_1 = from_a.unwrap().get("1");
            assert!(by_1.is_some());
            assert_eq!(by_1.unwrap(), "b");
        };

        let mut fsm = FSM::new("initial_state".to_string());
        fsm.add_action("a".to_string(),
                           "b".to_string(),
                           "1".to_string());
        check_fsm_action(&fsm);

        fsm.add_action("a".to_string(),
                           "b".to_string(),
                           "1".to_string());
        check_fsm_action(&fsm);
    }

    #[test]
    fn check_actions() {
        let mut fsm = FSM::new("initial_state".to_string());
        fsm.add_action("a".to_string(),"b".to_string(),"1".to_string());
        fsm.add_action("a".to_string(),"c".to_string(),"2".to_string());
        fsm.add_action("b".to_string(),"d".to_string(),"3".to_string());

        assert_eq!(fsm.next_state(&"a".to_string(), &"1".to_string()), Some(&"b".to_string()));
        assert_eq!(fsm.next_state(&"a".to_string(), &"3".to_string()), None);
        assert_eq!(fsm.next_state(&"a".to_string(), &"2".to_string()), Some(&"c".to_string()));
        assert_eq!(fsm.next_state(&"b".to_string(), &"3".to_string()), Some(&"d".to_string()));
        assert_eq!(fsm.next_state(&"b".to_string(), &"4".to_string()), None);
    }
}
