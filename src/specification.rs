use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use fsm::FSM;

use yaml_rust::YamlLoader;

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
    println!("READ {:?}", loaded);


}
