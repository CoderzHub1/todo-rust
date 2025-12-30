use std::io::{Write, stdin, stdout};
use todo::tasks::{manage_tasks::add_task, tasks::Task};
fn main() {
    let mut my_tasks: Vec<Task> = Vec::new();
    let mut name: String = String::new();
    print!("Enter task name: ");
    stdout().flush().expect("failed to flush stdout");
    stdin().read_line(&mut name).expect("Failed to read line");
    name = name.trim().to_string();
    let new_task: Task = Task {
        id: 1,
        name: name,
        completed: false,
    };
    add_task(&mut my_tasks, new_task).expect("Failed to add task");

    for task in &my_tasks {
        println!(
            "id:{}\nname:{}\ncompleted:{}",
            task.id, task.name, task.completed
        );
    }
}
