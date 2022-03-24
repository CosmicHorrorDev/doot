use std::{env, path::PathBuf};

use anyhow::Result;
use doot::run;

fn main() -> Result<()> {
    env_logger::init();

    let args = env::args().collect();
    let todo_path = PathBuf::from("todos.json");
    run(args, todo_path)?;

    Ok(())
}
