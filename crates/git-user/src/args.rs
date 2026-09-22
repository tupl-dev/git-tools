use clap::{Parser, Subcommand};

/// Represents CLI arguments.
#[derive(Clone, Debug, Parser)]
#[command(name = "git-user", about = "Manage Git users.")]
pub struct Args {
    /// CLI subcommand.
    #[command(subcommand)]
    pub command: Command,

    /// Specifies the config file path.
    #[arg(short, long, global = true, default_value = "~/.gitusers")]
    pub config: String,
}

/// Represents a CLI subcommand.
#[derive(Clone, Debug, Subcommand)]
pub enum Command {
    /// Subcommand for creating a new user profile.
    #[command(about = "Create a user", alias = "a")]
    Add(AddArgs),

    /// Subcommand for deleting a user.
    #[command(about = "Delete a user", alias = "d")]
    Delete {
        /// Specifies the user profile name.
        profile: String,
    },

    /// Subcommand for exporting the config.
    #[command(about = "Export config", alias = "e")]
    Export,

    /// Subcommand for listing users.
    #[command(about = "List users", alias = "l")]
    List,

    /// Subcommand for using a specific user.
    #[command(about = "Use user", alias = "u")]
    Use {
        /// Specifies the user profile name.
        profile: String,

        /// Specifies the git repository path.
        #[arg(short, long, default_value = ".")]
        repo: String,
    },
}

/// Represents the arguments for [`Command::Add`].
#[derive(Clone, Debug, clap::Args)]
pub struct AddArgs {
    /// Specifies `user.name`.
    pub name: String,

    /// Specifies `user.email`.
    pub email: String,

    /// Specifies the user profile name.
    #[arg(short, long)]
    pub profile: Option<String>,

    /// Specifies `user.signingKey`.
    #[arg(short = 'k', long)]
    pub signing_key: Option<String>,

    /// Specifies `user.sshCommand`.
    #[arg(short = 's', long)]
    pub ssh_command: Option<String>,

    /// Specifies `gpg.format`.
    #[arg(long)]
    pub gpg_format: Option<String>,
}
