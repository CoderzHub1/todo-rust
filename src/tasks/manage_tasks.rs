use crate::tasks::tasks::Task;
use bincode;
use sled::{self, Db};
use std::error::Error;

/// Add a new `Task` to both the provided `sled::Db` and the in-memory `tasks` vector.
///
/// This function performs the following steps:
/// 1. Serializes `new_task` using `bincode`.
/// 2. Uses the task's `id` converted to a big-endian byte array as the database key.
/// 3. Inserts the serialized value into the provided `sled::Db`.
/// 4. Appends `new_task` to the in-memory `tasks` `Vec`.
///
/// # Parameters
/// - `db`: Reference to the sled database where serialized `Task` values are stored.
/// - `tasks`: Mutable reference to an in-memory vector of `Task`; the new task is
///   pushed onto this vector after a successful database insert.
/// - `new_task`: The `Task` instance to add.
///
/// # Errors
/// Returns a boxed error (`Box<dyn Error>`) if any operation fails:
/// - `bincode::serialize` failure when serializing `new_task`.
/// - `sled::Db::insert` failure when writing to the database.
///
/// # Notes
/// - The function appends the task to the provided `tasks` vector only after the
///   database insert succeeds, keeping the in-memory state consistent with the DB.
/// - The DB key is derived from `new_task.id.to_be_bytes()`; callers relying on
///   a particular key layout should ensure IDs are assigned accordingly.
///
/// # Examples
/// ```no_run
/// let db = sled::open("tasks")?;
/// let mut tasks = Vec::new();
/// let new_task = Task { id: 1, name: String::from("Buy milk"), completed: false };
/// add_task(&db, &mut tasks, new_task)?;
/// ```
pub fn add_task(db: &Db, tasks: &mut Vec<Task>, new_task: Task) -> Result<(), Box<dyn Error>> {
    let encoded = bincode::serialize(&new_task)?;
    let key = new_task.id.to_be_bytes();
    db.insert(key, encoded)?;
    tasks.push(new_task);
    Ok(())
}
