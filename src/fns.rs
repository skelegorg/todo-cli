// we love modularity

pub struct Task {
    pub name: String,
    pub due: Option<i64>,
    pub created_on: i64,
}

pub fn add_to_file(_task: Task) {
    println!("hello");
}

