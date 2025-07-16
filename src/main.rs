mod task;
mod todo;
use todo::TodoApp;

fn main() {
    let mut app = TodoApp::new();
    let _ = app.run();
}
