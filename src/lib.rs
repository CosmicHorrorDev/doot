mod cli;
mod todos;

use todos::Todos;

use std::path::PathBuf;

use anyhow::{Context, Result};

pub fn run(args: Vec<String>, todo_path: PathBuf) -> Result<()> {
    let mut todos = Todos::load_from(todo_path)?;
    let args = cli::parse_args(args).context("Failed parsing args")?;
    log::info!("parsed args:\n{args:#?}");

    todo!("Do something with the parsed `args` and `todos`");

    todos.save()?;

    Ok(())
}
