// we love modularity

pub struct Task {
    pub name: String,
    pub due: Option<chrono::DateTime<chrono::Utc>>,
    pub created_on: chrono::DateTime<chrono::Utc>,
}

pub fn handle_input(opt: &str, args: Vec<String>) {
    match opt {
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
            let f_due_string = time_string_to_date_time(_current_time, due_string);
            let new_task: Task = Task {name: task_name, due: f_due_string, created_on: _current_time};
            add_task(new_task);
            return
        }
        "todo" => {
            println!("todo proced lmao");
        }
        _ => {
            println!("jekasd");
        }
    }
}

// convert the input time string to a Utc time obj for the due time
pub fn time_string_to_date_time(_current_time: chrono::DateTime<chrono::Utc>, _due_time: String) -> Option<chrono::DateTime<chrono::Utc>> {
    if _due_time == String::from("NONE") {
        return None
    }
    Some(chrono::Utc::now())
}

pub fn add_task(task: Task) {
    println!("adding task {}", task.name);
}
