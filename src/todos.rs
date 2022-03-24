use anyhow::Result;
use serde::{Deserialize, Serialize};

use std::{collections::HashMap, fs, path::PathBuf};

use crate::cli::SortBy;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low,
    Med,
    High,
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
    // Maps the title -> todo
    todos: HashMap<String, Todo>,
}

impl Todos {
    pub fn load_from(path: PathBuf) -> Result<Todos> {
        if path.is_file() {
            // Try loading the file
            let contents = fs::read_to_string(&path)?;
            let todos = serde_json::from_str(&contents)?;
            Ok(Todos { path, todos })
        } else {
            // No existing saved file. Make a new one
            Ok(Todos {
                path,
                todos: HashMap::new(),
            })
        }
    }

    pub fn save(&self) -> Result<()> {
        let json_str = serde_json::to_string(&self.todos)?;
        fs::write(&self.path, &json_str)?;

        Ok(())
    }

    pub fn add(&mut self, todo: Todo) -> anyhow::Result<()> {
        let title = &todo.title;
        if self.todos.contains_key(title) {
            anyhow::bail!("Title conflict with existing entry. Title: {title}");
        }

        let _ = self.todos.insert(title.to_owned(), todo);

        Ok(())
    }

    pub fn list(&self, maybe_sort_by: Option<SortBy>, reverse: bool) -> String {
        let mut values: Vec<_> = self.todos.values().collect();

        // Sort the entries before displaying
        if let Some(sort_by) = maybe_sort_by {
            match sort_by {
                SortBy::Title => values.sort_by_key(|todo| &todo.title),
                SortBy::Priority => values.sort_by_key(|todo| &todo.priority),
            }
        }

        if reverse {
            values.reverse();
        }

        todo!();
    }

    pub fn remove(&mut self, title: String) -> Result<()> {
        if self.todos.remove(&title).is_none() {
            anyhow::bail!("Failed removing entry. Couldn't find title: {title}");
        }

        Ok(())
    }
}
