use crate::tasks::tasks::Task;
use csv::Reader;
use std::error::Error;
use std::fs::File;

pub fn extract_tasks(filename: String) -> Result<Vec<Task>, Box<dyn Error>> {
    let mut tasks: Vec<Task> = vec![];
    let file_exists: bool = std::path::Path::new(filename.trim()).exists();
    if file_exists {
        let file = File::open(filename).expect("Cant find tasks.csv");
        let mut rdr = Reader::from_reader(file);

        for result in rdr.deserialize() {
            match result {
                Ok(task) => tasks.push(task),
                Err(e) => eprintln!("Error reading record: {}", e),
            }
        }
    } else {
        eprintln!("File {} doesnt exist", filename)
    }
    Ok(tasks)
}
