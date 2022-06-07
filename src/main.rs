// cli for quickly maintaining a to-do list of stuff to keep on your mind.

use std;
use std::env;
extern crate chrono;
mod fns;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd = &args[1];
    match cmd.as_str() {
        "add" => {
            let _current_time: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
            // cmd processing :sunglasses:
            if args[2].starts_with("-") {
                println!("Don't forget a task name. Try todo --help!");
                return
            }
            let mut task_name: String = args[2].to_owned();
            for n in 3..args.len() {
                if args[n].starts_with("-") {
                    break
                }
                task_name = format!("{} {}", task_name, args[n].to_owned());
            }
            println!("{}", task_name);
            return
        }
        _ => {
            println!("jekasd");
        }
    }
    println!("{:?}",args);
}
