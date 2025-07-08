use chrono::{DateTime, Utc};
use std::io;

struct Task {
    id: i32,
    description: String,
    due_datetime: DateTime<Utc>,
    completed: bool,
}

struct TodoApp {
    tasks: Vec<Task>,
}

impl TodoApp {
    fn new() -> Self {
        Self { tasks: Vec::new() }
    }
    fn gather_info(&self, opt: i32) -> Task {
        match opt {
            1 => {
                let h = io::stdin();
                println!("Let's create a new task. First, the description: ");
                let mut description = String::new();
                h.read_line(description).expect("Cannot read input: ");
                println!("Next, the date and time the task is due: ");
                let mut due_datetime = String::new();
                h.read_line(due_datetime).expect("Cannot read input: ");
                let due_datetime = DateTime::parse_from_rfc3339(due_datetime.as_str())
                    .unwrap()
                    .with_timezone(&Utc);
                Task {
                    id: 0,
                    description: description,
                    due_datetime: due_datetime,
                    completed: false,
                }
            }
            2 => {}
        }
    }
    fn add(&self) -> Result<(), String> {
        let h = io::stdin();
        println!("Let's create a new task. First, the description: ");
        let mut description = String::new();
        h.read_line(description).expect("Cannot read input: ");
        println!("Next, the date and time the task is due: ");
        let mut due_datetime = String::new();
        h.read_line(due_datetime).expect("Cannot read input: ");
        let due_datetime = DateTime::parse_from_rfc3339(due_datetime.as_str())
            .unwrap()
            .with_timezone(&Utc);
        let task = Task {
            id: 0,
            description: description,
            due_datetime: due_datetime,
            completed: false,
        };
    }
    fn modify(&self, task_id: i32, task_details: Task) -> Result<(), String> {}
    fn delete(&self, task_id: i32) -> Result<(), String> {}
    fn display(&self) -> () {}
    fn run(self) -> () {
        loop {
            println!(
                "Choose one of the following options by entering the number corresponding to it and pressing enter:"
            );
            println!("(1) Create a new task.");
            println!("(2) Mark an existing task as completed.");
            println!("(3) Modify an existing task.");
            println!("(4) Delete an existing task.");
            println!("(5) List all incomplete tasks.");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Input could not be read.");
            let opt: i32 = input.parse().expect("Could not parse input.");
            match opt {
                1 => self.add(),
            }
        }
    }
}

fn main() {
    let app = TodoApp::new();
    app.run();
}
