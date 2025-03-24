use rusqlite::Connection;
use std::io;

mod db;
use db::*;

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error while taking input from user");
    input
}

fn add_todo(conn: &Connection) {
    println!("Enter your task : ");
    let task = get_input().trim().to_owned();
    let _ = insert_task(conn, &task);
}

fn list_todo(conn: &Connection) {
    let _ =fetch_tasks(conn);
}

fn mark_as_done(conn: &Connection) {
    println!("Enter task's index : ");
    let index = get_input()
        .trim()
        .parse::<i32>()
        .expect("Unable to parse input to usize");
    let _ = mark_task_done(conn, index);
}

fn main() {
    println!("Welcome to Todo App");
    let conn = create_db().expect("Error while creating database");

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
            1 => add_todo(&conn),
            2 => list_todo(&conn),
            3 => mark_as_done(&conn),
            4 => {
                println!("Thanks for using Todo App");
                break;
            }
            _ => println!("Invalid choice"),
        }
    }

}
