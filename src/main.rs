// cli for quickly maintaining a to-do list of stuff to keep on your mind.

use std;
use std::env;
extern crate chrono;
mod fns;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        fns::handle_input("todo", args);
    } else {
        let cmd = &args[1].clone();
        fns::handle_input(cmd.as_str(), args);
    }
}
