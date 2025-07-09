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
        print!("Let's create a new task. First, the description: ");
        let mut description = String::new();
        h.read_line(description).expect("Cannot read input: ");
        print!("Next, the date and time the task is due (in RFC3339 format): ");
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
        self.tasks.push(task);
        Ok(())
    }
    fn modify(&self) -> Result<(), String> {
        print!("Enter id for the task that you would like to modify: ")
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        let id: i32 = input.parse().unwrap();
        // check that task with this id exists
        let task_exists = false;
        for t in &mut self.tasks {
            if t.id == id {
                let task = t;
                task_exists = true;
            }
        }
        if !task_exists {
            return Err("Could not find task.");
        }
        print!("Enter new task description (leave empty if unchanged): ")
        input.clear()
        io::stdin().read_line(&mut input);
        if input != "" {
            task.description = input;
        }
        print!("Enter new task due date and time in RFC3339 format (leave empty if unchanged): ")
        input.clear()
        io::stdin().read_line(&mut input);
        if input != "" {
            let due_datetime = DateTime::parse_from_rfc3339(input.as_str())
            .unwrap()
            .with_timezone(&Utc);
            task.due_datetime = due_datetime;
        }
        Ok(())
    }
    fn mark_as_completed(&mut self) -> Result<(), &str> {
        // get task id from user
        print!("Enter id for the task that you would like to mark as completed: ")
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Input could not be read.");
        let id: i32 = input.parse().unwrap();
        // can do binary search here since the ids are sorted
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = true;
                return Ok(());
            }
        }
        Err("Could not find task.")
    }
    fn delete(&self) -> Result<(), String> {
        // TODO
    }
    fn display(&self) -> () {}
    fn run(self) -> ! {
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
                2 => self.mark_as_completed(),
                3 => self.modify(),
                4 => self.delete(),
                _ => Ok(()),
            };
        }
    }
}

fn main() {
    let app = TodoApp::new();
    app.run();
}
