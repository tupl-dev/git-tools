use crate::todo_item::TodoItem;
use clap::Parser;

mod args;
mod config;
mod error;
mod git;
mod todo_item;

fn main() {
    let args = args::Args::parse();
    match handle_args(args) {
        Ok(code) => std::process::exit(code),
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        },
    }
}

fn handle_args(args: args::Args) -> Result<i32, error::AppError> {
    let repo_path = shellexpand::tilde(&args.repo);
    let repo = repo_path.as_ref();
    let branch = match args.branch {
        Some(b) => b,
        None => git::get_branch(repo)?,
    };
    let mut config = git::get_todo_config(repo)?;
    match args.subcommand {
        args::Sub::Add { item } => {
            config.add(&branch, TodoItem::new(item));
            git::write_todo_config(repo, &config)?;
        },
        args::Sub::Check { quiet } => {
            let completed = !config.get(&branch).any(|item| !item.completed);
            if !completed {
                if !quiet {
                    println!("Some items are incomplete");
                }
                return Ok(1);
            }
        },
        args::Sub::Clear { done } => {
            config.clear(&branch, done);
            git::write_todo_config(repo, &config)?;
        },
        args::Sub::Delete { indexes } => {
            config.delete(&branch, indexes);
            git::write_todo_config(repo, &config)?;
        },
        args::Sub::Complete { indexes } => {
            config.toggle(&branch, indexes);
            git::write_todo_config(repo, &config)?;
        },
        args::Sub::List { undone } => {
            config.get(&branch).enumerate().for_each(|(idx, item)| {
                if undone && item.completed {
                    return;
                }
                println!("{}", item.to_string(idx));
            });
        },
    }
    Ok(0)
}
