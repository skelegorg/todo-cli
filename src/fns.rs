// we love modularity

pub struct Task {
    pub name: String,
    pub due: Option<chrono::DateTime<chrono::Utc>>,
    pub created_on: chrono::DateTime<chrono::Utc>,
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
