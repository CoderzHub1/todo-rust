use crate::tasks::tasks::Task;
use bincode;
use sled::{self, Db};
use std;
use std::error::Error;

/// Extract all tasks stored in the provided sled `Db`.
///
/// This function iterates over every key/value pair in `db`, deserializes each
/// value into a `Task` using `bincode`, and collects the results into a `Vec<Task>`
/// which is returned. The order of tasks in the returned vector follows the order
/// produced by the database iterator.
///
/// Errors:
/// - Returns an error if iterating or reading entries from the database fails
///   (sled errors).
/// - Returns an error if deserializing any stored value into `Task` fails
///   (bincode errors).
///
/// # Examples
///
/// ```rust
/// let tasks = extract_tasks(&db)?;
/// for task in tasks {
///     println!("{:?}", task);
/// }
/// ```
pub fn extract_tasks(db: &Db) -> Result<Vec<Task>, Box<dyn Error>> {
  let mut tasks: Vec<Task> = Vec::new();
  for item in db.iter() {
    let (_, value) = item?;
    let task: Task = bincode::deserialize(&value)?;
    tasks.push(task);
  }
  Ok(tasks)
}


/// Update the completion `status` of the task with `task_id` in both the
/// provided in-memory `tasks_vec` and the sled `db`.
///
/// Behavior:
/// - Converts `task_id` to a big-endian byte array to use as the key for `db`.
/// - If a value exists for that key, deserializes it into a `Task`, updates the
///   `completed` field to `status`, re-serializes and inserts the updated value
///   back into the database, and replaces the corresponding entry in `tasks_vec`.
///
/// Parameters:
/// - `tasks_vec`: mutable reference to the in-memory list of tasks.
/// - `db`: reference to the sled database containing serialized `Task` values.
/// - `task_id`: 1-based identifier used as the database key.
/// - `status`: new completion status to set for the task.
///
/// Errors:
/// - Returns any error produced by `db.get`/`db.insert` (sled errors).
/// - Returns any error produced by `bincode` during serialization or deserialization.
///
/// Safety notes:
/// - If the key is not present in the database this function is a no-op and
///   returns `Ok(())`.
/// - This function updates `tasks_vec[(task_id - 1) as usize]`. It will panic if
///   `task_id` is 0 or if the resulting index is out of bounds for `tasks_vec`.
///   Callers should ensure `tasks_vec` contains the task with the given `task_id`
///   or otherwise guard against out-of-bounds access.
///
/// # Examples
///
/// ```rust
/// let mut my_tasks:Vec<Task> = vec![];
/// let db = sled::open("database_name")?;
/// let task_id:u32 = 1;
/// let status = true;
/// update_task(&mut my_tasks, &db, task_id, status)?;
///
/// ```

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
