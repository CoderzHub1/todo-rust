use std::io::{Write, stdin, stdout};
use todo::tasks::{Task, manage_tasks::add_task};
fn main() {
    let mut my_tasks: Vec<Task> = Vec::new();
    let mut name: String = String::new();
    print!("Enter task name: ");
    stdout().flush().expect("failed to flush stdout");
    stdin().read_line(&mut name).expect("Failed to read line");
    name.trim().to_string();
    let new_task: Task = Task {
        id: 1,
        name: name,
        completed: false,
    };

    let success = add_task(&mut my_tasks, new_task);
    if success {
        println!("Task added successfully!");

        for task in &my_tasks {
            println!(
                "id:{}\nname:{}\ncompleted:{}",
                task.id, task.name, task.completed
            );
        }
    } else {
        println!("Some error occured")
    }
}
