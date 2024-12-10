use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
struct Task {
    id: Uuid,
    title: String,
    description: String,
    completed: bool,
}

fn main() {
    println!("Welcome to this file based task manager written in rust");
}
