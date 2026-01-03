use crate::tasks::tasks::Task;
use sled::{self, Db};
use std;
use std::error::Error;

pub fn extract_tasks(db: &Db) -> Result<Vec<Task>, Box<dyn Error>> {
    let mut tasks: Vec<Task> = Vec::new();
    for item in db.iter() {
        let (_, value) = item?;
        let task: Task = bincode::deserialize(&value)?;
        tasks.push(task);
    }
    Ok(tasks)
}
