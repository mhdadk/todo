use chrono::NaiveDateTime;

pub struct Task {
    pub id: i32,
    pub description: String,
    pub due_datetime: NaiveDateTime,
    pub completed: bool,
}
