use std::collections::HashMap;
use std::collections::HashSet;

pub struct PetriNet {
    pub places: HashSet<String>,
    pub transitions: HashSet<String>,
    pub flow_places: HashMap<String,HashSet<String>>,
    pub flow_transitions: HashMap<String,HashSet<String>>
}


impl PetriNet {

    pub fn new() -> PetriNet {
        PetriNet{ places: HashSet::new(),
                  transitions: HashSet::new(),
                  flow_places: HashMap::new(),
                  flow_transitions: HashMap::new() }
    }

    pub fn flow_place_transition(&mut self, place:String, transition:String) {
        self.transitions.insert(transition.clone());
        match self.flow_places.get(&place) {
            None => {
                let mut flow_transitions = HashSet::new();
                flow_transitions.insert(transition);
                self.flow_places.insert(place.clone(), flow_transitions);
                self.places.insert(place);
            },
            Some(_) => {
                let mut flow_transitions = self.flow_places.get_mut(&place).unwrap();
                flow_transitions.insert(transition);
            }
        }
    }

    pub fn flow_transition_place(&mut self, transition:String, place:String) {
        self.places.insert(place.clone());
        match self.flow_transitions.get(&transition) {
            None => {
                let mut flow_places = HashSet::new();
                flow_places.insert(place);
                self.flow_transitions.insert(transition.clone(), flow_places);
                self.transitions.insert(transition);
            },
            Some(_) => {
                let mut flow_places = self.flow_transitions.get_mut(&transition).unwrap();
                flow_places.insert(place);
            }
        }
    }
}
