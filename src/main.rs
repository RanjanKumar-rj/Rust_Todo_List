use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{self, Read};

#[derive(Deserialize, Serialize, Debug)]
struct Task {
    name: String,
    done: bool,
}

const FILE_PATH: &str = "tasks.json";

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error while taking input from user");
    input
}

/// Load tasks from the JSON file
fn load_tasks() -> Vec<Task> {
    let mut file = match OpenOptions::new().read(true).open(FILE_PATH) {
        Ok(file) => file,
        Err(_) => return Vec::new(), // Return empty if file doesn't exist
    };

    let mut data = String::new();
    file.read_to_string(&mut data).unwrap_or(0);

    serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
}

/// Save tasks to the JSON file
fn save_tasks(tasks: &Vec<Task>) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(tasks)?;
    fs::write(FILE_PATH, json)?;
    Ok(())
}

fn add_todo(tasks: &mut Vec<Task>) {
    println!("Enter your task : ");
    let task = get_input().trim().to_owned();
    tasks.push(Task {
        name: task,
        done: false,
    });
}

fn list_todo(tasks: &mut Vec<Task>) {
    for (index, task) in tasks.iter().enumerate() {
        println!(
            "{}. {} [{}]",
            index + 1,
            task.name,
            if task.done { "Done" } else { "Pending" }
        );
    }
}

fn mark_as_done(tasks: &mut Vec<Task>) {
    println!("Enter task's index : ");
    let index = get_input()
        .trim()
        .parse::<usize>()
        .expect("Unable to parse input to usize");
    if index > 0 && index < tasks.len() {
        tasks[index - 1].done = true;
    } else {
        println!("Invalid task index");
    }
}

fn main() {
    println!("Welcome to Todo App");
    let mut tasks = load_tasks();

    loop {
        println!("\n1. Add Todo");
        println!("2. List Todo");
        println!("3. Mark as done");
        println!("4. Exit");
        println!("Enter your choice : ");

        let choice = get_input()
            .trim()
            .parse::<u8>()
            .expect("Unable to parse input to usize");

        match choice {
            1 => add_todo(&mut tasks),
            2 => list_todo(&mut tasks),
            3 => mark_as_done(&mut tasks),
            4 => {
                println!("Thanks for using Todo App");
                break;
            }
            _ => println!("Invalid choice"),
        }
    }

    save_tasks(&tasks).expect("Failed to save data in the JSON file");
}
