extern crate cortazar;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        println!("The first argument is {}", args[1]);
        cortazar::specification::load(&args[1]);
    } else if args.len() > 2 {
        println!("Too many arguments provided {}", args.len());
    } else {
        println!("No arguments provided");
    }
}
