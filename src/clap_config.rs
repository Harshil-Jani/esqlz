use clap::{Parser, Subcommand};

// Define the CLI structure
#[derive(Parser, Debug)]
#[command(name = "esqlz")]
#[command(about = "Easequelize is a TUI to manage sequelize migrations easily.", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

// Define the subcommands
#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Select migration files and run the up migration")]
    Up{
        file_name: Option<String>,
    },
    #[command(about = "Select migration files and run the down migration")]
    Down{
        file_name: Option<String>,
    },
    #[command(about = "Show status for recent migrations")]
    Status,
    #[command(about = "Create a new migration file template")]
    Generate {
        msg: String,
    },
}
