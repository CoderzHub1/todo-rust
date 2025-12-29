use crate::tasks::Task;

pub fn add_task() {
    println!("You accessed your tasks");
    let new_task: Task = Task {
        id: 1,
        name: String::from("abcd"),
        completed: false,
    };
    println!(
        "id:{}\nname:{}\ncompleted:{}",
        new_task.id, new_task.name, new_task.completed
    );
}
