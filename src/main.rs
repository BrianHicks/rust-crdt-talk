mod crdt;
mod document;
mod replica;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use replica::Replica;
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
        let mut replica = self.load_replica().context("could not load replica")?;

        let changed = tracing_texray::examine(tracing::info_span!("run")).in_scope(|| {
            self.command
                .run(&mut replica)
                .context("could not run command")
        })?;

        if changed {
            self.store_replica(&replica)
                .context("could not store replica")?;
        }

        Ok(())
    }

    fn load_replica(&self) -> Result<Replica> {
        if !self.store_path.exists() {
            return Ok(Replica::new());
        }

        let file = std::fs::File::open(&self.store_path)
            .with_context(|| format!("could not open `{}`", self.store_path.display()))?;

        let replica: Replica = serde_json::from_reader(file)
            .with_context(|| format!("could not read `{}` as JSON", self.store_path.display()))?;

        Ok(replica)
    }

    fn store_replica(&self, replica: &Replica) -> Result<()> {
        let file = std::fs::File::create(&self.store_path)
            .with_context(|| format!("could not create `{}`", self.store_path.display()))?;

        serde_json::to_writer(file, replica)
            .with_context(|| format!("could not write JSON to `{}`", self.store_path.display()))?;

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

impl Command {
    fn run(&self, replica: &mut Replica) -> Result<bool> {
        match self {
            Self::List => {
                for (id, task) in replica.tasks() {
                    println!("{id} {task}");
                }

                Ok(false)
            }

            Self::Add { description } => {
                let uuid = replica.add_task(description.join(" "));

                eprintln!("Added task");
                println!("{}", uuid);

                Ok(true)
            }

            _ => anyhow::bail!("Unimplemented"),
        }
    }
}

fn main() {
    tracing_texray::init();

    let cli = Cli::parse();

    if let Err(err) = cli.run() {
        eprintln!("Error: {:#}", err);
        std::process::exit(1);
    }
}
