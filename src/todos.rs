use serde::{Deserialize, Serialize};

use std::path::PathBuf;

use crate::cli::SortBy;
use crate::error::Result;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Priority {
    High,
    Med,
    Low,
}

impl Priority {
    pub fn new(s: String) -> Option<Priority> {
        match s.as_str() {
            "high" => Some(Priority::High),
            "med" => Some(Priority::Med),
            "low" => Some(Priority::Low),
            _ => None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Todo {
    pub priority: Priority,
    pub title: String,
    pub desc: Option<String>,
}

pub struct Todos {
    path: PathBuf,
    todos: Vec<Todo>,
}

impl Todos {
    pub fn load_from(path: PathBuf) -> Result<Todos> {
        todo!("Implement loading todos");
    }

    pub fn save(&self) -> Result<()> {
        todo!("Implement saving todos");
    }

    pub fn add(&mut self, priority: Priority, title: String, desc: Option<String>) {
        todo!("Implement adding a todo");
    }

    pub fn list(&self, sort_by: Option<SortBy>, reverse: bool) -> String {
        todo!("Implement listing todos");
    }

    pub fn remove(&mut self, title: String) -> Result<()> {
        todo!("Implement removing a todo");
    }
}
