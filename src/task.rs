use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub description: String,
    pub due_datetime: NaiveDateTime,
    pub completed: bool,
}
