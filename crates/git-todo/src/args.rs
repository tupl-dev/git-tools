use clap::{Parser, Subcommand};

/// Represents CLI arguments.
#[derive(Clone, Debug, Parser)]
#[command(name = "git-todo", about = "Manage TODO lists for git.")]
pub struct Args {
    /// Specifies the branch to operate on.
    #[arg(short, long, global = true)]
    pub branch: Option<String>,

    /// Specifies the git repository path.
    #[arg(short, long, default_value = ".")]
    pub repo: String,

    /// CLI subcommand.
    #[command(subcommand)]
    pub subcommand: Sub,
}

/// Represents CLI subcommands.
#[derive(Clone, Debug, Subcommand)]
pub enum Sub {
    /// Subcommand for adding a new item.
    #[command(about = "Add a new TODO item", alias = "a")]
    Add {
        /// Specifies the item content.
        item: String,
    },

    /// Subcommand for checking TODO items status.
    #[command(about = "Checks if there are still TODO items undone", alias = "ch")]
    Check {
        /// Specifies whether to suppress output.
        #[arg(short, long)]
        quiet: bool,
    },

    /// Subcommand for clearing TODO items.
    #[command(about = "Clears TODO items", alias = "cl")]
    Clear {
        /// Specifies whether to clear only done items.
        #[arg(short, long)]
        done: bool,
    },

    /// Subcommand for completing items.
    #[command(about = "Complete TODO items", alias = "c")]
    Complete {
        /// Specifies the item indexes to complete.
        indexes: Vec<usize>,
    },

    /// Subcommand for deleting items.
    #[command(about = "Delete TODO items", alias = "d")]
    Delete {
        /// Specifies the item indexes to delete.
        indexes: Vec<usize>,
    },

    /// Subcommand for listing items.
    #[command(about = "List TODO items", alias = "l")]
    List {
        /// Specifies whether to show only undone items.
        #[arg(short, long)]
        undone: bool,
    },
}
