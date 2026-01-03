use todo::{
    prompts::prompts::prompts,
    tasks::{extract_tasks::extract_tasks, manage_tasks::add_task, tasks::Task},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // -------------------BASIC INITIALIZATION-----------------------------
    let db = sled::open("tasks")?;
    let mut my_tasks: Vec<Task> = extract_tasks(&db).expect("cant extract tasks from the sled db");
    let mut last_id = 0;
    if let Some(last_task) = my_tasks.last() {
        last_id = last_task.id;
    }

    // ----------------MAIN APPLICATION LOOP----------------------------
    loop {
        let cmd = prompts("Enter the command: ");

        match cmd.as_str() {
            "add" => {
                let name = prompts("Enter task name: ");
                let new_task: Task = Task {
                    id: last_id + 1,
                    name: name,
                    completed: false,
                };
                add_task(&db, &mut my_tasks, new_task).expect("Failed to add task");
                last_id += 1;
            }

            "list" => {
                for task in &my_tasks {
                    println!(
                        "id:{}\nname:{}\ncompleted:{}",
                        task.id, task.name, task.completed
                    );
                }
            }

            "exit" => {
                return Ok(());
            }

            _ => println!("Invalid command"),
        }
    }
}
