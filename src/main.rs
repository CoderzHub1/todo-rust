use todo::{
    prompts::prompts::prompts,
    tasks::{extract_tasks::{extract_tasks, update_task}, manage_tasks::add_task, tasks::Task},
};
use owo_colors::OwoColorize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // -------------------BASIC INITIALIZATION-----------------------------
  let db = sled::open("tasks")?;
  let mut my_tasks: Vec<Task> = extract_tasks(&db).expect("cant extract tasks from the sled db");
  let mut last_id = 0;
  if let Some(last_task) = my_tasks.last() {
      last_id = last_task.id;
  }

  // ------------------MAIN APPLICATION LOOP----------------------------
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
          println!("");
          println!("{}: {}", "ID".bright_red().bold(), task.id.to_string().bright_blue());
          println!("{}: {}", "NAME".bright_red().bold(), task.name.to_string().green());
          println!("{}: {}", "STATUS".bright_red().bold(), task.completed.to_string().green());
          println!("");
        }
    }

    "update"=>{
      let task_id:u32 = prompts("Enter the task id to change: ").parse()?;
      let mut status_str = prompts("Enter the status[C/n]:");
      status_str.make_ascii_lowercase();
      let mut _status:bool = false;
      match status_str.as_str(){
        "c"=>{
          _status = true;
        }
        "n"=>{
          _status = false;
        }
        _=>{
          eprintln!("Invalid State encountered!")
        }
      }

      update_task(&mut my_tasks, &db, task_id, _status)?;

    }

    "refresh"=>{
      my_tasks = extract_tasks(&db).expect("cant extract tasks from the sled db");
    }

    "exit" => {
        return Ok(());
    }

    _ => println!("Invalid command"),
}
}
}
