use super::task::Task;
use chrono::NaiveDateTime;
use std::io::{self, Write};

use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct TodoApp {
    tasks: Vec<Task>,
    next_id: i32,
}

impl TodoApp {
    pub fn new() -> Self {
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
        print!("Next, the date and time the task is due (MM/DD/YYYY HH:MM AM/PM): ");
        io::stdout().flush().unwrap();
        let mut due_datetime = String::new();
        io::stdin()
            .read_line(&mut due_datetime)
            .map_err(|_| "Could not read due date and time.".to_string())?;
        // let x = due_datetime.trim();
        // let y = NaiveDateTime::parse_from_str(due_datetime.trim(), "%m/%d/%Y %I:%M %p");
        // let z = y.map_err(|_| "Could not parse due date and time.")?;
        let due_datetime = NaiveDateTime::parse_from_str(due_datetime.trim(), "%m/%d/%Y %I:%M %p")
            .map_err(|_| "Could not parse due date and time.".to_string())?;
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
            .map_err(|_| "Failed to read line".to_string())?;
        let id: i32 = input
            .trim()
            .parse()
            .map_err(|_| "Invalid id input".to_string())?;

        // need the .iter_mut() instead of .iter() so that task_opt is &mut T and not
        // &T so that task can be modified later
        let task = self
            .tasks
            .iter_mut()
            .find(|task| task.id == id)
            .ok_or("Could not find task.".to_string())?;

        input.clear();
        print!("Enter new task description (leave empty if unchanged): ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .map_err(|_| "Failed to read line")?;
        let new_desc = input.trim();
        if !new_desc.is_empty() {
            task.description = new_desc.to_string();
        }

        input.clear();
        print!(
            "Enter new date and time the task is due (MM/DD/YYYY HH:MM AM/PM) (leave empty if unchanged): "
        );
        io::stdout().flush().unwrap();
        let mut due_datetime = String::new();
        io::stdin()
            .read_line(&mut due_datetime)
            .map_err(|_| "Could not read due date and time.".to_string())?;
        let due_datetime = due_datetime.trim();
        if !due_datetime.is_empty() {
            task.due_datetime = NaiveDateTime::parse_from_str(due_datetime, "%m/%d/%Y %I:%M %p")
                .map_err(|_| "Could not parse due date and time.".to_string())?;
        }
        Ok(())
    }
    fn mark_as_completed(&mut self) -> Result<(), String> {
        // get task id from user
        print!("Enter id for the task that you would like to mark as completed: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input) // Err(Error)
            .map_err(|_| "Input could not be read.".to_string())?; // Err(String)
        let id: i32 = input
            .trim()
            .parse()
            .map_err(|_| "Invalid id input".to_string())?;
        let task = self
            .tasks
            .iter_mut()
            .find(|task| task.id == id)
            .ok_or("Could not find task.".to_string())?;
        task.completed = true;
        Ok(())
    }
    fn delete(&mut self) -> Result<(), String> {
        print!("Enter id for the task that you would like to delete: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|_| "Input could not be read.")?;
        let id: i32 = input
            .trim()
            .parse()
            .map_err(|_| "Invalid id input".to_string())?;
        self.tasks.retain(|task| task.id != id);
        Ok(())
    }
    fn display(&self) -> Result<(), String> {
        for task in self.tasks.iter().filter(|t| !t.completed) {
            println!("====================================");
            println!("Task {}", task.id);
            println!("Task description: {}", task.description);
            println!("Task due date and time: {}", task.due_datetime);
            println!("Completed? {}", if task.completed { "Yes" } else { "No" });
            println!("====================================")
        }
        io::stdout().flush().unwrap();
        Ok(())
    }
    fn save_to_file(&self, path: &str) -> Result<(), String> {
        let json = serde_json::to_string_pretty(&self)
            .map_err(|e| format!("Serialization error: {}", e))?;
        let mut file =
            fs::File::create(path).map_err(|e| format!("Failed to create file: {}", e))?;
        file.write_all(json.as_bytes())
            .map_err(|e| format!("Write error: {}", e))?;
        Ok(())
    }
    fn load_from_file(path: &str) -> Result<Self, String> {
        let content =
            fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;
        serde_json::from_str(&content).map_err(|e| format!("Deserialization error: {}", e))
    }
    pub fn run(&mut self) -> Result<(), String> {
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
                } else {
                    eprintln!("Could not parse option. Please try again.")
                }
            } else {
                eprintln!("Could not read input. Please try again.");
            }
        }
    }
}
