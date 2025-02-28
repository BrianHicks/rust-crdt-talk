mod crdt;
mod replica;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Parser)]
#[clap(about = "A simple task manager using CRDTs", version)]
struct Cli {
    #[clap(subcommand)]
    command: Command,

    /// Path to the database file
    #[clap(long, global = true, default_value = "tasks.json")]
    store_path: PathBuf,
}

impl Cli {
    fn run(&self) -> Result<()> {
        println!("{:#?}", self);

        Ok(())
    }
}

#[derive(Debug, Subcommand)]
enum Command {
    /// List all tasks
    List,
    /// Add a new task
    Add {
        /// Description of the task
        description: Vec<String>,
    },
    /// Update the description of an existing task
    Update {
        /// UUID of the task to update
        id: Uuid,
        /// New description of the task
        description: String,
    },
    /// Mark a task as complete or incomplete
    Complete {
        /// UUID of the task to update
        id: Uuid,
        /// Mark as complete (true) or incomplete (false)
        complete: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    if let Err(err) = cli.run() {
        eprintln!("Error: {:#}", err);
        std::process::exit(1);
    }
}
