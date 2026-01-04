use crate::tasks::tasks::Task;
use bincode;
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

pub fn update_task(tasks_vec:&mut Vec<Task>, db: &Db, task_id:u32, status:bool) -> Result<(), Box<dyn Error>>{
  let task_id_bytes = task_id.to_be_bytes();
  if let Some(val) = db.get(task_id_bytes)?{
    let mut decoded:Task = bincode::deserialize(&val)?;
    decoded.completed = status;
    let encoded = bincode::serialize(&decoded)?;
    db.insert(task_id_bytes, encoded)?;
    tasks_vec[(task_id-1) as usize] = decoded;
  }
  Ok(())
}
