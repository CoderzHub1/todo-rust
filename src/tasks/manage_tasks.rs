use crate::tasks::tasks::Task;
use std::error::Error;
use std::fs::OpenOptions;

pub fn add_task(tasks: &mut Vec<Task>, new_task: Task) -> Result<(), Box<dyn Error>> {
    let file_exists = std::path::Path::new("tasks.csv").exists();
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .write(true)
        .open("tasks.csv")?;
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(!file_exists)
        .from_writer(file);

    wtr.serialize(&new_task)?;
    wtr.flush()?;
    tasks.push(new_task);
    Ok(())
}
