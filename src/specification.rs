use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use fsm::FSM;

use yaml_rust::YamlLoader;
use yaml_rust::yaml::Yaml;


pub fn load(path:&String) {
    let path = Path::new(path);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}",
                           display,
                           Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    let content = match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}",
                           display,
                           Error::description(&why)),
        Ok(_) => s,
    };

    let loaded = YamlLoader::load_from_str(&content).unwrap();
    if loaded.len() == 1 {
        let description = loaded[0].as_hash().unwrap();
        let initial_state = description.get(&Yaml::String(String::from("initial-state")));
        let fsm = match initial_state {
            Some(&Yaml::String(_)) => {
                let transitions = description.get(&Yaml::String(String::from("transitions")));
                match transitions {
                    Some(transitions) => {
                        let initial_state_str = String::from(initial_state.unwrap().as_str().unwrap());
                        let mut fsm = FSM::new(initial_state_str);

                        for transition in transitions.as_vec().unwrap() {
                            let transition_triple =  transition.as_vec().unwrap();
                            let from = String::from(transition_triple[0].as_str().unwrap());
                            let by =  String::from(transition_triple[1].as_str().unwrap());
                            let to = String::from(transition_triple[2].as_str().unwrap());
                            fsm.add_transition(from,to,by);
                        }
                        fsm
                    },
                    None => panic!("Missing transitions in FSM specification")
                }
            },
            Some(v) => panic!("Unexpected input value for initial state {:?}",v),
            None => panic!("Missing initial state in FSM specification")
        };

        println!("FSM READ!!! {}",&fsm);
    }
}
