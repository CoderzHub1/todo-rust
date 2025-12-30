use std::io::{Write, stdin, stdout};
use todo::tasks::{extract_tasks::extract_tasks, manage_tasks::add_task, tasks::Task};
fn main() {
    let mut my_tasks: Vec<Task> =
        extract_tasks("tasks.csv".to_string()).expect("cant extract tasks from the csv file");
    // fetching data from tasks.csv to my_tasks
    let mut last_id = 0;
    if let Some(last_task) = my_tasks.last() {
        last_id = last_task.id;
    }

    for i in &my_tasks {
        println!("id:{}\nname:{}\ncompleted:{}", i.id, i.name, i.completed);
    }

    let mut name: String = String::new();
    print!("Enter task name: ");
    stdout().flush().expect("failed to flush stdout");
    stdin().read_line(&mut name).expect("Failed to read line");
    name = name.trim().to_string();
    let new_task: Task = Task {
        id: last_id + 1,
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
