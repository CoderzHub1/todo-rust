use crate::tasks::tasks::Task;
use bincode;
use sled::{self, Db};
use std::error::Error;

pub fn add_task(db: &Db, tasks: &mut Vec<Task>, new_task: Task) -> Result<(), Box<dyn Error>> {
    let encoded = bincode::serialize(&new_task)?;
    let key = new_task.id.to_be_bytes();
    db.insert(key, encoded)?;
    tasks.push(new_task);
    Ok(())
}
