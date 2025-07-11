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
        // need the .map_err() because .read_line() does not return Err(String) but
        // something else
        io::stdin()
            .read_line(&mut description)
            .map_err(|_| "Could not read description.".to_string())?;
        print!("Next, the date the task is due (in RFC3339 format): ");
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
            println!("(0) Exit.");
            let mut input = String::new();
            if let Ok(_) = io::stdin().read_line(&mut input) {
                if let Ok(opt) = input.trim().parse() {
                    match opt {
                        1 => {
                            if let Err(e) = self.add() {
                                eprintln!("Error adding new task: {}", e);
                            }
                        }
                        2 => {
                            if let Err(e) = self.mark_as_completed() {
                                eprintln!("Error marking task as completed: {}", e);
                            }
                        }
                        3 => {
                            if let Err(e) = self.modify() {
                                eprintln!("Error modifying task: {}", e);
                            }
                        }
                        4 => {
                            if let Err(e) = self.delete() {
                                eprintln!("Error deleting task: {}", e);
                            }
                        }
                        5 => {
                            if let Err(e) = self.display() {
                                eprintln!("Error showing tasks: {}", e);
                            }
                        }
                        0 => {
                            println!("Exiting...");
                            return Ok(());
                        }
                        _ => eprintln!("Please choose a number between 1 and 5 (inclusive)."),
                    };
                }
            } else {
                eprintln!("Could not read input. Please try again.")
            }
        }
    }
}

fn main() -> () {
    let mut app = TodoApp::new();
    let _ = app.run();
}
