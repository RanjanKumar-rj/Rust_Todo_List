use clap::{Parser, Subcommand};
use rusqlite::Result;

mod db;
use db::*;

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple CLI Todo App", version = "1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the database
    Init,

    /// Add a new todo task
    Add {
        #[arg(short, long)]
        task: String,
    },

    /// List all todo tasks
    List,

    /// Mark a task as done
    Done {
        #[arg(short, long)]
        id: i32,
    },

    /// Delete a task
    Delete {
        #[arg(short, long)]
        id: i32,
    },
}

fn main() -> Result<()> {
    println!("Welcome to Todo App");

    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            create_db()?;
        }
        Commands::Add { task } => {
            insert_task(task)?;
        }
        Commands::List => {
            fetch_tasks()?;
        }
        Commands::Done { id } => {
            mark_task_done(*id)?;
            println!("Task {} marked as done!", *id);
        }
        Commands::Delete { id } => {
            delete_task(*id)?;
            println!("Task {} deleted!", id);
        }
    }

    Ok(())
}
