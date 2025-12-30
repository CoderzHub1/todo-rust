use crate::tasks::tasks::Task;
use std::error::Error;
use std::fs::OpenOptions;

pub fn add_task(tasks: &mut Vec<Task>, new_task: Task) -> Result<(), Box<dyn Error>> {
    let file = std::
}
