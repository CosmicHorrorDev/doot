mod cli;
mod error;
mod todos;

use std::path::PathBuf;

use crate::todos::Todos;

fn main() {
    env_logger::init();

    let args: Vec<_> = std::env::args().collect();
    log::info!("unparsed args:\n{args:?}");
    let args = cli::parse_args(args).expect("Invalid cli args");
    log::info!("parsed args:\n{args:#?}");

    let todo_path = PathBuf::from("todos.json");
    let mut todos = Todos::load_from(todo_path).expect("Failed to load todos");

    todo!("Do something with the parsed `args` and `todos`");

    todos.save().expect("Failed to save todos file");
}
