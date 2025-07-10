use chrono::{DateTime, Utc};
use std::io::{self, Write};

struct Task {
    id: i32,
    description: String,
    due_datetime: DateTime<Utc>,
    completed: bool,
}

struct TodoApp {
    tasks: Vec<Task>,
    next_id: i32,
}

impl TodoApp {
    fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 0,
        }
    }
    fn add(&mut self) -> Result<(), String> {
        print!("Let's create a new task. First, the description: ");
        io::stdout().flush().unwrap();
        let mut description = String::new();
        io::stdin()
            .read_line(&mut description)
            .expect("Cannot read input: ");
        print!("Next, the date and time the task is due (in RFC3339 format): ");
        io::stdout().flush().unwrap();
        let mut due_datetime = String::new();
        io::stdin()
            .read_line(&mut due_datetime)
            .expect("Cannot read input: ");
        let due_datetime = DateTime::parse_from_rfc3339(due_datetime.trim())
            .unwrap()
            .with_timezone(&Utc);
        let task = Task {
            id: self.next_id,
            description: description,
            due_datetime: due_datetime,
            completed: false,
        };
        self.tasks.push(task);
        self.next_id += 1;
        Ok(())
    }
    fn modify(&mut self) -> Result<(), String> {
        print!("Enter id for the task that you would like to modify: ");
        // flush stdout buffer so that the line above prints before asking for user
        // input
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let id: i32 = input
            .trim()
            .parse()
            .map_err(|_| "Invalid id input".to_string())?;

        // need the .iter_mut() instead of .iter() so that task_opt is &mut T and not
        // &T so that task can be modified later
        let task_opt = self.tasks.iter_mut().find(|task| task.id == id);
        let task = match task_opt {
            Some(x) => x,
            None => return Err("Could not find task.".to_string()),
        };

        input.clear();
        print!("Enter new task description (leave empty if unchanged): ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let new_desc = input.trim();
        if !new_desc.is_empty() {
            task.description = new_desc.to_string();
        }

        input.clear();
        print!("Enter new task due date and time in RFC3339 format (leave empty if unchanged): ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let new_due = input.trim();
        if !new_due.is_empty() {
            task.due_datetime = DateTime::parse_from_rfc3339(new_due)
                .map_err(|_| "Invalid date format.".to_string())?
                .with_timezone(&Utc);
        }

        Ok(())
    }
    fn mark_as_completed(&mut self) -> Result<(), String> {
        // get task id from user
        print!("Enter id for the task that you would like to mark as completed: ");
        io::stdout().flush().unwrap();
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
        Err("Could not find task.".to_string())
    }
    fn delete(&mut self) -> Result<(), String> {
        print!("Enter id for the task that you would like to delete: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|_| "Input could not be read.")?;
        let id: i32 = input.parse().unwrap();
        self.tasks.retain(|task| task.id != id);
        Ok(())
    }
    fn display(&self) -> Result<(), String> {
        for task in &self.tasks {
            println!("Task {}", task.id);
            println!("Task description: {}", task.description);
            println!("Task due date and time {}", task.due_datetime);
            println!("Completed? {}", if task.completed { "Yes" } else { "No" });
            println!("====================================")
        }
        io::stdout().flush().unwrap();
        Ok(())
    }
    fn run(&mut self) -> Result<(), String> {
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
            let opt: i32 = input.trim().parse().expect("Could not parse input.");
            match opt {
                1 => self.add(),
                2 => self.mark_as_completed(),
                3 => self.modify(),
                4 => self.delete(),
                5 => self.display(),
                _ => return Err("Incorrect input. please try again.".to_string()),
            };
        }
    }
}

fn main() {
    let mut app = TodoApp::new();
    app.run();
}
