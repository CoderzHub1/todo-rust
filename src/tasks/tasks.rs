use serde::Serialize;

#[derive(Serialize)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub completed: bool,
}
