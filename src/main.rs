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
            let mut final_n = 0;
            let mut task_name: String = args[2].to_owned();
            for n in 3..args.len() {
                if args[n].starts_with("-") {
                    final_n = n;
                    break
                }
                task_name = format!("{} {}", task_name, args[n].to_owned());
            }
            let mut due_string: String = String::new();
            if args.contains(&String::from("--due")) {
                for n in final_n..args.len() {
                    due_string = format!("{} {}", due_string, args[n].to_owned());
                }
            } else {
                due_string = String::from("NONE")
            }
            let f_due_string = fns::time_string_to_date_time(_current_time, due_string);
            let new_task: fns::Task = fns::Task {name: task_name, due: f_due_string, created_on: _current_time};
            fns::add_task(new_task);
            return
        }
        _ => {
            println!("jekasd");
        }
    }
}
