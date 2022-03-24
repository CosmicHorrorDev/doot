use crate::todos::Priority;

// CLI args should have something like the following subcommands
// -> doot add (high|med|low) TITLE [DESCRIPTION]
// e.g.
// $ doot add high 'Very Important!' 'Cram for test'
// $ doot add med 'Take out the trash'
//
// -> doot list [(-s|--sort-by) (priority|title)] [-r|--reverse]
// e.g.
// $ doot list
// $ doot list --sort-by title
// $ doot list -s priority -r
//
// -> doot remove TITLE
// e.g.
// $ doot remove 'Take out the trash'  # Done and done
pub fn parse_args(args: Vec<String>) -> Option<Args> {
    todo!("Implement arg parsing");
}

#[derive(Debug, PartialEq)]
pub struct Args {
    command: Command,
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Add(AddArgs),
    List(ListArgs),
    Remove(RemoveArgs),
}

#[derive(Debug, PartialEq)]
pub struct AddArgs {
    priority: Priority,
    title: String,
    description: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct ListArgs {
    sort_by: SortBy,
    reverse: bool,
}

#[derive(Debug, PartialEq)]
pub struct RemoveArgs {
    title: String,
}

#[derive(Debug, PartialEq)]
pub enum SortBy {
    Priority,
    Title,
}

impl SortBy {
    fn new(s: String) -> Option<SortBy> {
        match s.as_str() {
            "priority" => Some(SortBy::Priority),
            "title" => Some(SortBy::Title),
            _ => None,
        }
    }

    fn default() -> SortBy {
        Self::Priority
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_args_fails() {
        let args = vec!["doot".into()];
        let parsed = parse_args(args);

        assert_eq!(parsed, None);
    }

    #[test]
    fn missing_sort_by_value_fails() {
        let args = vec!["doot".into(), "list".into(), "--sort-by".into()];
        let parsed = parse_args(args);

        assert_eq!(parsed, None);
    }

    #[test]
    fn add_works() {
        let args = vec![
            "doot".into(),
            "add".into(),
            "high".into(),
            "some title".into(),
        ];
        let parsed = parse_args(args);

        assert_eq!(
            parsed,
            Some(Args {
                command: Command::Add(AddArgs {
                    priority: Priority::High,
                    title: "some title".into(),
                    description: None
                })
            })
        );
    }

    #[test]
    fn list_works() {
        let args = vec!["doot".into(), "list".into()];
        let parsed = parse_args(args);

        assert_eq!(
            parsed,
            Some(Args {
                command: Command::List(ListArgs {
                    sort_by: SortBy::default(),
                    reverse: false
                })
            })
        );
    }

    #[test]
    fn remove_works() {
        let args = vec!["doot".into(), "remove".into(), "some title".into()];
        let parsed = parse_args(args);

        assert_eq!(
            parsed,
            Some(Args {
                command: Command::Remove(RemoveArgs {
                    title: "some title".into()
                })
            })
        );
    }
}
